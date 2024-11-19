<template>
	<LayoutTile
		title="Device Progressive Test"
		description="Test the connected USB RS232 device step by step."
	>
		<div class="mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
			<div class="grid grid-cols-1 gap-x-8 gap-y-6">
				<!-- Device Info Display -->
				<div>
					<label block text-sm text-white font-semibold leading-6>Connected Device</label>
					<div mt="2.5">
						<p class="text-white">
							<strong>Device:</strong> {{ devicePath }}
						</p>
					</div>
				</div>

				<!-- Current Test Step -->
				<div>
					<label block text-sm text-white font-semibold leading-6>Current Test</label>
					<div mt="2.5">
						<p class="text-white">
							<strong>Step:</strong>
							{{ testSteps[currentTestStepIndex]?.description }}
						</p>
					</div>
					<div mt="2.5">
						<p class="text-white">
							<strong>Command:</strong>
							{{
								testSteps[currentTestStepIndex]?.commands[
									currentCommandIndex
								]?.command
							}}
							<strong>Parameters:</strong>
							{{
								testSteps[currentTestStepIndex]?.commands[
									currentCommandIndex
								]?.parameters
							}}
						</p>
					</div>
				</div>

				<!-- Base Status Display -->
				<div v-if="baseStatus">
					<h3 class="text-lg text-white font-semibold">
						Base Device Status:
					</h3>
					<pre class="rounded-md bg-gray-800 p-4 text-white">
						{{ JSON.stringify(baseStatus, null, 2) }}
					</pre>
				</div>

				<!-- User Feedback -->
				<div v-if="awaitingFeedback">
					<p class="text-white font-semibold">
						Did the device respond correctly to the command:
						<strong>{{
							testSteps[currentTestStepIndex]?.commands[
								currentCommandIndex
							]?.command
						}}</strong>?
					</p>
					<div class="mt-2 flex gap-4">
						<Btn @click="recordResult(true)">
							Yes
						</Btn>
						<Btn @click="recordResult(false)">
							No
						</Btn>
					</div>
				</div>

				<!-- Start/Next Test Button -->
				<div v-else>
					<Btn v-if="!testFinished" @click="startTest">
						{{ testInProgress ? "Next Command" : "Start Test" }}
					</Btn>
					<Btn v-else class="bg-emerald-600" @click="saveTestResults">
						Save Results
					</Btn>
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
	import { onMounted, ref } from "vue";
	import { useRoute } from "vue-router";

	const route = useRoute();

	// Test Steps Definition
	interface TestCommand {
		command: string
		parameters: string
		executeReadAfter?: boolean
	}

	interface TestStep {
		description: string
		commands: TestCommand[]
	}

	const testSteps: TestStep[] = [
		{
			description: "Check Base Device Status",
			commands: [
				{ command: "S0\n\rR", parameters: "" },
				{ command: "R", parameters: "1" }
			]
		},
		{
			description: "Turn LEDs On and Off",
			commands: [
				{ command: "L", parameters: "1" },
				{ command: "L", parameters: "0" }
			]
		},
		{
			description: "Test LED Set Selection",
			commands: [
				{ command: "P", parameters: "0" },
				{ command: "P", parameters: "1" }
			]
		},
		{
			description: "Test RGB Colors",
			commands: [
				{ command: "C", parameters: "FF,00,00", executeReadAfter: true }, // Red
				{ command: "C", parameters: "00,FF,00", executeReadAfter: true }, // Green
				{ command: "C", parameters: "00,00,FF", executeReadAfter: true } // Blue
			]
		},
		{
			description: "Test Flash Function",
			commands: [{ command: "F", parameters: "" }]
		}
	];

	const baseStatus = ref({});
	const devicePath = ref(route.query.path as string);
	const currentTestStepIndex = ref(0);
	const currentCommandIndex = ref(0);
	const testInProgress = ref(false);
	const awaitingFeedback = ref(false);
	const testResults = ref<Record<string, any>[]>([]);
	const testFinished = ref(false);

	// Initialize Serial Port
	const openPort = async () => {
		try {
			await invoke("open_port", { path: devicePath.value });
			console.log("Port opened successfully.");
		} catch (error) {
			console.error("Error opening port:", error);
		}
	};

	const runNextCommand = async () => {
		if (awaitingFeedback.value) {
			console.log("Awaiting feedback. Command execution paused.");
			return;
		}

		const step = testSteps[currentTestStepIndex.value];
		const command = step.commands[currentCommandIndex.value];

		try {
			const fullCommand = `${command.command}${command.parameters}\r\n`;
			awaitingFeedback.value = true;

			// Write command to port
			const response = await invoke("write_to_port", {
				path: devicePath.value,
				data: fullCommand
			});

			console.log(`Command sent: ${fullCommand}`);

			// Add a delay before attempting to read the response
			await new Promise((resolve) => setTimeout(resolve, 1000)); // Adjust the delay as needed

			console.log(`Device response: ${response}`);
			testResults.value.push({ command: fullCommand, result: response });

			// Proceed to the next command or test step
			if (currentCommandIndex.value < step.commands.length - 1) {
				currentCommandIndex.value += 1;
			} else {
				currentCommandIndex.value = 0;
				if (currentTestStepIndex.value < testSteps.length - 1) {
					currentTestStepIndex.value += 1;
				} else {
					testFinished.value = true;
				}
			}
		} catch (error) {
			console.error("Error sending command:", error);
		} finally {
			awaitingFeedback.value = false;
			testInProgress.value = false;
		}
	};

	const startTest = () => {
		if (testFinished.value)
			return;
		testInProgress.value = true;
		runNextCommand();
	};

	// Record User Feedback
	const recordResult = (success: boolean) => {
		const step = testSteps[currentTestStepIndex.value];
		testResults.value.push({
			description: step.description,
			success
		});

		awaitingFeedback.value = false;

		// Proceed to next step or finish
		if (!testFinished.value) {
			runNextCommand();
		}
	};

	// Save Results to JSON File (or backend)
	const saveTestResults = async () => {
		try {
			await invoke("save_results", { results: testResults.value });
		// alert("Test results saved successfully.");
		} catch (error) {
			console.error("Error saving results:", error);
		}
	};

	// Lifecycle
	onMounted(() => {
		openPort();
	});
</script>
