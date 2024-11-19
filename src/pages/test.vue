<template>
	<LayoutTile
		title="Strobe LED Test Wizard"
		description="Follow the steps to test the strobe LED device."
	>
		<div class="mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
			<div class="grid grid-cols-1 gap-x-8 gap-y-6">
				<!-- Device Info -->
				<div>
					<label
						class="block text-sm text-white font-semibold leading-6"
					>
						Connected Device
					</label>
					<div class="mt-2.5">
						<p class="text-white">
							<strong>Device:</strong> {{ devicePath }}
						</p>
					</div>
				</div>

				<!-- Current Test -->
				<div>
					<label
						class="block text-sm text-white font-semibold leading-6"
					>
						Current Test
					</label>
					<div class="mt-2.5">
						<p class="text-white">
							<strong>Step:</strong> {{ currentTest.description }}
						</p>
					</div>
				</div>

				<!-- User Feedback -->
				<div v-if="awaitingFeedback">
					<p class="text-white font-semibold">
						{{ currentPrompt }}
					</p>
					<div class="mt-2 flex gap-4">
						<Btn @click="recordFeedback(true)">
							Yes
						</Btn>
						<Btn @click="recordFeedback(false)">
							No
						</Btn>
					</div>
				</div>

				<!-- Test Results -->
				<div v-if="testFinished" class="mt-4">
					<h3 class="text-lg text-white font-semibold">
						Test Results:
					</h3>
					<pre class="rounded-md bg-gray-800 p-4 text-white">
			  {{ JSON.stringify(testResults, null, 2) }}
			</pre>
				</div>
			</div>
		</div>
	</LayoutTile>
</template>

<script lang="ts" setup>
	import { invoke } from "@tauri-apps/api/core";
	import { computed, onMounted, ref } from "vue";
	import { useRoute } from "vue-router";

	const route = useRoute();
	const devicePath = ref(route.query.path as string);

	const testSteps = [
		{
			description: "Test RED LED",
			commands: [
				{ command: "CFF,00,00\r\nL1", type: "on" },
				{ command: "L0", type: "off" },
				{ command: "F0A,32", type: "flash" },
				{ command: "L0", type: "off" }
			],
			prompts: ["Is the RED LED on?", "Is the RED LED flashing?"]
		},
		{
			description: "Test GREEN LED",
			commands: [
				{ command: "C00,FF,00\r\nL1", type: "on" },
				{ command: "L0", type: "off" },
				{ command: "F0A,32", type: "flash" },
				{ command: "L0", type: "off" }
			],
			prompts: ["Is the GREEN LED on?", "Is the GREEN LED flashing?"]
		},
		{
			description: "Test BLUE LED",
			commands: [
				{ command: "C00,00,FF\r\nL1", type: "on" },
				{ command: "L0", type: "off" },
				{ command: "F0A,32", type: "flash" },
				{ command: "L0", type: "off" }
			],
			prompts: ["Is the BLUE LED on?", "Is the BLUE LED flashing?"]
		}
	];

	const currentStepIndex = ref(0);
	const currentCommandIndex = ref(0);
	const testResults = ref([]);
	const awaitingFeedback = ref(false);
	const testFinished = ref(false);

	const currentTest = computed(() => testSteps[currentStepIndex.value]);
	const currentCommand = computed(
		() => currentTest.value.commands[currentCommandIndex.value]
	);
	const currentPrompt = computed(
		() => currentTest.value.prompts[Math.floor(currentCommandIndex.value / 2)]
	);

	async function sendCommand(command: string) {
		try {
			const res = await invoke("write_to_port", {
				path: devicePath.value,
				data: `${command}\r\n`
			});
			return res;
		} catch (error) {
			console.error("Error sending command:", error);
		}
	}

	async function executeNextCommand() {
		const command = currentCommand.value.command;
		const res = await sendCommand(command);
		console.log(res);

		if (currentCommandIndex.value % 2 === 0) {
			awaitingFeedback.value = true;
		} else {
			moveToNextCommand();
		}
	}

	function moveToNextCommand() {
		if (currentCommandIndex.value < currentTest.value.commands.length - 1) {
			currentCommandIndex.value += 1;
		} else if (currentStepIndex.value < testSteps.length - 1) {
			currentCommandIndex.value = 0;
			currentStepIndex.value += 1;
		} else {
			testFinished.value = true;
		}
		awaitingFeedback.value = false;
		if (!testFinished.value) {
			executeNextCommand();
		}
	}

	function recordFeedback(success: boolean) {
		testResults.value.push({
			step: currentTest.value.description,
			prompt: currentPrompt.value,
			success
		});
		awaitingFeedback.value = false;
		moveToNextCommand();
	}

	async function startTest() {
		try {
			await invoke("open_port", { path: devicePath.value });
			console.log("Port opened successfully.");
		} catch (error) {
			if (error.includes("already open")) {
				await invoke("close_port", { path: devicePath.value });
				await invoke("open_port", { path: devicePath.value });
				console.log("Reopened port. Proceeding...");
			} else {
				console.error("Failed to open port:", error);
				return;
			}
		}

		try {
			// Send initial commands and start the test
			await sendCommand("S0");
			const result = await sendCommand("R");
			console.log(result);
			// const initialData = await readData();
			// console.log("Initial read:", initialData);
			await executeNextCommand();
		} catch (error) {
			console.error("Error during test sequence:", error);
		}
	}

	onMounted(async () => {
		await startTest();
	});
</script>
