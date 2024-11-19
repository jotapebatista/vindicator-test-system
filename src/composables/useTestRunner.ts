import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

export interface TestCommand {
	command: string
	parameters: string
	executeReadAfter?: boolean
}

export interface TestStep {
	description: string
	commands: TestCommand[]
}

export function useTestRunner(devicePath: string, testSteps: TestStep[]) {
	const currentTestStepIndex = ref(0);
	const currentCommandIndex = ref(0);
	const testResults = ref<Record<string, any>[]>([]);
	const testFinished = ref(false);
	const testInProgress = ref(false);

	const runNextCommand = async () => {
		if (testFinished.value)
			return;

		const step = testSteps[currentTestStepIndex.value];
		const command = step.commands[currentCommandIndex.value];
		const fullCommand = `${command.command}${command.parameters}\r\n`;

		try {
			await invoke("write_to_port", { portName: devicePath, data: fullCommand });

			if (command.executeReadAfter) {
				const status = await invoke<string>("read_device_status", {
					portName: devicePath
				});
				testResults.value.push({ command: command.command, result: status });
			}

			// Update indices
			if (currentCommandIndex.value < step.commands.length - 1) {
				currentCommandIndex.value++;
			} else if (currentTestStepIndex.value < testSteps.length - 1) {
				currentCommandIndex.value = 0;
				currentTestStepIndex.value++;
			} else {
				testFinished.value = true;
			}
		} catch (error) {
			console.error("Error during command execution:", error);
		} finally {
			testInProgress.value = false;
		}
	};

	const startTest = () => {
		if (testInProgress.value || testFinished.value)
			return;
		testInProgress.value = true;
		runNextCommand();
	};

	const saveResults = async () => {
		try {
			await invoke("save_results", { results: testResults.value });
			// alert("Test results saved successfully.");
		} catch (error) {
			console.error("Error saving results:", error);
		}
	};

	return {
		currentTestStepIndex,
		currentCommandIndex,
		testResults,
		testFinished,
		testInProgress,
		startTest,
		saveResults
	};
}
