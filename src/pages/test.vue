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

		<!-- Feedback Modal -->
		<FeedbackModal
			v-if="awaitingFeedback"
			:visible="awaitingFeedback"
			:question="currentPrompt"
			@respond="recordFeedback"
		/>
	</LayoutTile>
</template>

<script lang="ts" setup>
	import { invoke } from "@tauri-apps/api/core";
	import { computed, onMounted, ref } from "vue";
	import { useRoute } from "vue-router";

	const route = useRoute();
	const devicePath = ref(route.query.path as string);

	const testSteps = createTestSteps();
	const currentStepIndex = ref(0);
	const currentCommandIndex = ref(0);
	const testResults = ref<TestResult[]>([]);
	const awaitingFeedback = ref(false);
	const testFinished = ref(false);
	const dipSwitchStatus = ref<string>("");
	const awaitingDipSwitchFeedback = ref(false);

	const currentTest = computed(() => testSteps[currentStepIndex.value]);
	const currentCommand = computed(
		() => currentTest.value.commands[currentCommandIndex.value]
	);
	const currentPrompt = computed(() =>
		getPrompt(currentTest.value, currentCommandIndex.value)
	);

	async function startTest() {
		try {
			await ensurePortOpen();
			await executeCommandSequence(["S0", "R", "P0"]);
			await executeNextCommand();
		} catch (error) {
			console.error("Error during test sequence:", error);
		}
	}

	async function ensurePortOpen() {
		try {
			await invoke("open_port", { path: devicePath.value });
			console.log("Port opened successfully.");
		} catch (error: any) {
			if (error.includes("already open")) {
				await invoke("close_port", { path: devicePath.value });
				await invoke("open_port", { path: devicePath.value });
				console.log("Reopened port. Proceeding...");
			} else {
				throw new Error(`Failed to open port: ${error}`);
			}
		}
	}

	async function executeCommandSequence(commands: string[]) {
		for (const command of commands) {
			const response = await sendCommand(command);
			console.log(response);
		}
	}

	async function sendCommand(command: string) {
		try {
			const res = await invoke("write_to_port", {
				path: devicePath.value,
				data: `${command}\r\n`
			});
			return res;
		} catch (error) {
			console.error("Error sending command:", error);
			throw error;
		}
	}

	async function executeNextCommand() {
		const command = currentCommand.value.command;
		console.log(command);

		await sendCommand(command);

		if (command.includes("R") || command.includes("r")) {
			await handleDipSwitches();
		}

		if (isAwaitingFeedback(currentCommandIndex.value)) {
			awaitingFeedback.value = true;
		} else {
			moveToNextCommand();
		}
	}

	async function handleDipSwitches() {
		const response = await sendCommand("R");
		const dipSwitches = parseDipSwitchStatus(response);
		dipSwitchStatus.value = dipSwitches;
		awaitingDipSwitchFeedback.value = true;
	}

	function parseDipSwitchStatus(response: string): string {
		const match = response.match(/.*,(.*?),.*$/);
		if (match && match[1]) {
			return match[1]; // Extracts the DIP switch status
		}
		return "";
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

	function moveToNextCommand() {
		if (currentCommandIndex.value < currentTest.value.commands.length - 1) {
			currentCommandIndex.value++;
		} else if (currentStepIndex.value < testSteps.length - 1) {
			currentStepIndex.value++;
			currentCommandIndex.value = 0;
		} else {
			testFinished.value = true;
		}
		if (!testFinished.value)
			executeNextCommand();
	}

	function isAwaitingFeedback(commandIndex: number): boolean {
		return commandIndex % 2 === 0;
	}

	function getPrompt(test: TestStep, commandIndex: number): string {
		return test.prompts[Math.floor(commandIndex / 2)];
	}

	function createTestSteps(): TestStep[] {
		return [
			// Main LED Tests
			createTestStep("Test RED LED", "C02,00,00", [
				"Is the RED LED on?",
				"Is the RED LED flashing?"
			]),
			createTestStep("Test GREEN LED", "C00,02,00", [
				"Is the GREEN LED on?",
				"Is the GREEN LED flashing?"
			]),
			createTestStep("Test BLUE LED", "C00,00,02", [
				"Is the BLUE LED on?",
				"Is the BLUE LED flashing?"
			]),

			// DIP Switches Test Step
			createTestStep("Test DIP Switches", "R", [
				"Change DIP switch 1 to the opposite position and press Enter to continue.",

				"Change DIP switch 2 to the opposite position and press Enter to continue.",

				"Change DIP switch 3 to the opposite position and press Enter to continue.",

				"Change DIP switch 4 to the opposite position and press Enter to continue.",

				"Change DIP switch 5 to the opposite position and press Enter to continue."
			])
		];
	}

	function createTestStep(
		description: string,
		colorCommand: string,
		prompts: string[]
	): TestStep {
		return {
			description,
			commands: [
				{ command: `${colorCommand}\r\nL1`, type: "on" },
				{ command: "L0", type: "off" },
				{ command: "F0A,32", type: "flash" },
				{ command: "L0", type: "off" }
			],
			prompts
		};
	}

	onMounted(startTest);

	interface Command {
		command: string
		type: "on" | "off" | "flash"
	}

	interface TestStep {
		description: string
		commands: Command[]
		prompts: string[]
	}

	interface TestResult {
		step: string
		prompt: string
		success: boolean
	}
</script>
