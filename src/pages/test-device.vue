<template>
	<LayoutTile
		title="Vindicator Test Wizard"
		description="Follow the steps to test the device."
	>
		<div
			class="relative mx-auto max-w-3xl px-6 py-8 lg:max-w-4xl lg:mr-0 lg:px-8 lg:py-12"
		>
			<!-- Always visible buttons -->
			<div class="mb-4 text-center">
				<button
					class="rounded-md bg-blue-600 px-6 py-3 text-sm text-white font-semibold hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-blue-800 dark:hover:bg-blue-700"
					@click="startTest"
				>
					Start Test
				</button>
			</div>

			<!-- Message Logs Section -->
			<div>
				<div class="mb-1 px-1">
					<span v-if="GDSnrInTest" class="text-gray-300 mb-2"
						><span class="font-bold">Device SNR:</span>
						{{ GDSnrInTest }}
					</span>
				</div>

				<div
					class="h-64 overflow-y-auto border border-gray-200 dark:border-gray-700 shadow-sm rounded-lg bg-gray-800 p-4"
					ref="logContainer"
				>
					<div
						v-if="!testCompleted"
						v-for="(message, index) in logs"
						:key="index"
						class="mb-3 flex items-start gap-3"
					>
						<div
							:class="{
								'bg-blue-100 text-blue-700':
									message.type === 'Sent',
								'bg-green-100 text-green-700':
									message.type === 'Received',
							}"
							class="flex-shrink-0 flex items-center justify-center w-8 h-8 rounded-full"
						>
							<Icon
								:name="
									message.type === 'Sent'
										? 'ph:paper-plane-right'
										: 'ph:envelope-open'
								"
								size-4
							/>
						</div>
						<div class="flex-grow">
							<p
								class="text-sm font-medium mb-1"
								:class="{
									'text-blue-600': message.type === 'Sent',
									'text-green-600':
										message.type === 'Received',
								}"
							>
								[{{ message.type }}]
							</p>
							<p class="text-white text-sm">{{ message.text }}</p>
						</div>
					</div>
					<div v-else>
						<h2 class="text-xl font-semibold mb-4 text-white">
							{{ testStatus }}
						</h2>
						<ul class="space-y-2">
							<li
								:class="{
									'text-green-500': testPassed.leds,
									'text-red-500': !testPassed.leds,
								}"
								class="flex items-center"
							>
								<Icon
									:name="
										testPassed.leds
											? 'ph:check-circle'
											: 'ph:x-circle'
									"
									size-5
									class="mr-2"
								/>
								LEDs:
								{{ testPassed.leds ? "Passed" : "Failed" }}
							</li>
							<li
								:class="{
									'text-green-500': testPassed.dipSwitches,
									'text-red-500': !testPassed.dipSwitches,
								}"
								class="flex items-center"
							>
								<Icon
									:name="
										testPassed.dipSwitches
											? 'ph:check-circle'
											: 'ph:x-circle'
									"
									size-5
									class="mr-2"
								/>
								Dip Switches:
								{{
									testPassed.dipSwitches ? "Passed" : "Failed"
								}}
							</li>
							<li
								:class="{
									'text-green-500': testPassed.ctrlSignals,
									'text-red-500': !testPassed.ctrlSignals,
								}"
								class="flex items-center"
							>
								<Icon
									:name="
										testPassed.ctrlSignals
											? 'ph:check-circle'
											: 'ph:x-circle'
									"
									size-5
									class="mr-2"
								/>
								Control Signals:
								{{
									testPassed.ctrlSignals ? "Passed" : "Failed"
								}}
							</li>
							<li
								:class="{
									'text-green-500': testPassed.doorSensor,
									'text-red-500': !testPassed.doorSensor,
								}"
								class="flex items-center"
							>
								<Icon
									:name="
										testPassed.doorSensor
											? 'ph:check-circle'
											: 'ph:x-circle'
									"
									size-5
									class="mr-2"
								/>
								Door Sensor:
								{{
									testPassed.doorSensor ? "Passed" : "Failed"
								}}
							</li>
						</ul>
					</div>
				</div>
			</div>

			<FeedbackModal
				v-if="feedbackVisible"
				:visible="feedbackVisible"
				:question="feedbackQuestion"
				:confirmBtn="isQuestion"
				@respond="handleFeedback"
			/>

			<!-- Fixed action buttons -->
			<div
				v-if="GDSnrInTest"
				class="fixed bottom-4 right-4 z-50 space-x-2"
			>
				<button
					class="rounded-md bg-red-600 px-6 py-3 text-sm text-white font-semibold hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 dark:bg-red-800 dark:hover:bg-red-700"
					@click="emergencyStop"
				>
					Stop Test
				</button>
			</div>
		</div>
	</LayoutTile>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRoute } from "vue-router";
import { test } from "@antfu/eslint-config";

interface LedStatus {
	on: boolean;
	flash: boolean;
	backup: boolean;
	current: number;
	power: number;
}

interface BaseReading {
	current: number;
	inputVoltage: number;
	capacitorVoltage: number;
	temperature: number;
}

interface TestReport {
	gdSnr: string;
	leds: {
		RED: LedStatus;
		GREEN: LedStatus;
		BLUE: LedStatus;
	};
	dipSwitches: Record<number, boolean>;
	ctrlSignal: Record<number, boolean>;
	doorSignal: boolean;
	baseReadings: BaseReading;
}

const testReport: TestReport = {
	gdSnr: "",
	leds: {
		RED: { on: false, flash: false, backup: false, current: 0, power: 0 },
		GREEN: { on: false, flash: false, backup: false, current: 0, power: 0 },
		BLUE: { on: false, flash: false, backup: false, current: 0, power: 0 },
	},
	baseReadings: {
		current: 0,
		inputVoltage: 0,
		capacitorVoltage: 0,
		temperature: 0,
	},
	dipSwitches: { 1: false, 2: false, 3: false, 4: false, 5: false },
	ctrlSignal: { 1: false, 2: false },
	doorSignal: false,
};

enum DeviceCommands {
	SELECT_DEVICE = "S",
	READ_STATUS = "R",
	SET_LED_BACKUP = "P",
	SET_LED_FLASH = "F",
	SET_LED_STATE = "L",
}

const LedColors = {
	RED: "CFF,00,00",
	GREEN: "C00,FF,00",
	BLUE: "C00,00,FF",
};

const route = useRoute();
const devicePath = ref(route.query.path as string);
const logs = ref<{ type: string; text: string }[]>([]);
const logContainer = ref<HTMLDivElement | null>(null);
const feedbackVisible = ref(false);
const feedbackQuestion = ref("");
const feedbackCallback = ref<((response: boolean) => void) | null>(null);
const isQuestion = ref(true);
const GDSnrInTest = ref("");
const testPassed = ref({
	leds: false,
	dipSwitches: false,
	ctrlSignals: false,
	doorSensor: false,
});

const testCompleted = ref(false);

/* ---- Utility Functions ---- */
function logMessage(type: "Sent" | "Received", text: string) {
	logs.value.push({ type, text });
}

async function sendCommand(command: string): Promise<string> {
	try {
		const formattedCommand = command + "\r\n";
		logMessage("Sent", formattedCommand);
		const response = await invoke("write_to_port", {
			path: devicePath.value,
			data: formattedCommand,
		});
		logMessage("Received", response as string);
		return response as string;
	} catch (error) {
		console.error(`Error sending command "${command}":`, error);
		throw new Error(`Failed to send command: ${command}`);
	}
}

async function showFeedback(question: string): Promise<boolean> {
	feedbackVisible.value = true;
	feedbackQuestion.value = question;

	return new Promise((resolve) => {
		feedbackCallback.value = (response: boolean) => {
			feedbackVisible.value = false;
			resolve(response);
		};
	});
}

function handleFeedback(response: boolean) {
	if (feedbackCallback.value) feedbackCallback.value(response);
}

/* ---- Device Report Parsing ---- */
function parseDeviceReport(response: string) {
	try {
		if (!response.startsWith("r")) {
			throw new Error("Invalid response format");
		}

		const parts = response.split(",");
		const gdSnr = parts[0]?.trim();
		const current = Number(parts[1]?.trim());
		const inputVoltage = Number(parts[2]?.trim());
		const capacitorVoltage = Number(parts[3]?.trim());
		const dipSwitchesHex = parts[4]?.trim();
		const doorSensor = parts[6]?.trim() === "1";
		const controlSignals = parts[7]?.trim();
		const degTemperature = Number(parts[10]?.trim());

		const dipSwitchesBinary = parseInt(dipSwitchesHex, 16)
			.toString(2)
			.padStart(8, "0");
		const dipSwitches = {
			1: dipSwitchesBinary[7] === "0",
			2: dipSwitchesBinary[6] === "0",
			3: dipSwitchesBinary[5] === "0",
			4: dipSwitchesBinary[4] === "0",
			5: dipSwitchesBinary[3] === "0",
		};

		const controlSignalsBinary = parseInt(controlSignals || "0", 16)
			.toString(2)
			.padStart(2, "0")
			.split("")
			.reduce((acc, bit, index) => {
				acc[index + 1] = bit === "0";
				return acc;
			}, {} as Record<number, boolean>);

		return {
			gdSnr,
			baseReadings: {
				current,
				inputVoltage,
				capacitorVoltage,
				temperature: degTemperature,
			},
			dipSwitches,
			ctrlSignal: controlSignalsBinary,
			doorSignal: doorSensor,
		};
	} catch (error) {
		console.error("Error parsing device report:", error);
		throw error;
	}
}

/* ---- Test Flow Functions ---- */
async function testLedColors(color: keyof typeof LedColors) {
	try {
		await sendCommand(LedColors[color]);
		await sendCommand("L1");
		const response = await showFeedback(`Is the ${color} LED on?`);
		const statusResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const parsedReport = parseDeviceReport(statusResponse);

		testReport.leds[color] = {
			...testReport.leds[color],
			on: response,
			current: parsedReport.baseReadings.current,
			power: parsedReport.baseReadings.capacitorVoltage,
		};
	} catch (error) {
		console.error(`Error testing ${color} LED:`, error);
	}
}

async function testBackupLeds(color: keyof typeof LedColors) {
	try {
		await sendCommand(LedColors[color]);
		await sendCommand("L1");
		await sendCommand("P1");
		const response = await showFeedback(`Is the Backup ${color} LED on?`);

		testReport.leds[color] = {
			...testReport.leds[color],
			backup: response,
		};
		await sendCommand("L0");
		await sendCommand("P0");
	} catch (error) {
		console.error(`Error testing ${color} LED:`, error);
	}
}

async function flashLedColors(color: keyof typeof LedColors) {
	try {
		await sendCommand(LedColors[color]);
		await sendCommand("FA");
		const response = await showFeedback(
			`Is the flash ${color} LED flashing?`
		);

		testReport.leds[color] = {
			...testReport.leds[color],
			flash: response,
		};
	} catch (error) {
		console.error(`Error testing ${color} LED:`, error);
	}
}

async function testDeepSwitch(switchN: number) {
	try {
		isQuestion.value = false;
		const initialResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const initialState = parseDeviceReport(initialResponse);
		const initialSwitchState = initialState.dipSwitches[switchN];

		const userResponse = await showFeedback(
			`Please change the position of switch ${switchN}.`
		);

		const updatedResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const updatedState = parseDeviceReport(updatedResponse);
		const updatedSwitchState = updatedState.dipSwitches[switchN];

		if (initialSwitchState !== updatedSwitchState) {
			testReport.dipSwitches[switchN] = true;
		} else {
			testReport.dipSwitches[switchN] = false;
		}
		isQuestion.value = true;
	} catch (error) {
		console.error(`Error testing switch ${switchN}:`, error);
	}
}

async function testCtrlSignal(ctrlSignalN: number) {
	try {
		isQuestion.value = false;
		const initialResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const initialState = parseDeviceReport(initialResponse);
		const initalCtrlSignalState = initialState.ctrlSignal[ctrlSignalN];

		const userResponse = await showFeedback(
			`Please press the Control Signal ${ctrlSignalN} Button.`
		);

		const updatedResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const updatedState = parseDeviceReport(updatedResponse);
		const updatedCtrlSignalState = updatedState.ctrlSignal[ctrlSignalN];

		if (initalCtrlSignalState !== updatedCtrlSignalState) {
			testReport.ctrlSignal[ctrlSignalN] = true;
			console.log(`Switch ${ctrlSignalN} state changed successfully.`);
		} else {
			testReport.ctrlSignal[ctrlSignalN] = false;
			console.log(`Switch ${ctrlSignalN} state did not change.`);
		}
		isQuestion.value = true;
	} catch (error) {
		console.error(`Error testing switch ${ctrlSignalN}:`, error);
	}
}

async function testDoorSensor() {
	try {
		isQuestion.value = false;
		const initialResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const initialState = parseDeviceReport(initialResponse);
		const initalDoorState = initialState.doorSignal;
		console.log("Intial State Saved:", initalDoorState);

		const userResponse = await showFeedback(
			`Please press the Door Sensor Button.`
		);

		const updatedResponse = await sendCommand(DeviceCommands.READ_STATUS);
		const updatedState = parseDeviceReport(updatedResponse);
		console.log("Updated State Saved:", updatedState.doorSignal);
		const updatedDoorState = updatedState.doorSignal;

		if (initalDoorState !== updatedDoorState) {
			testReport.doorSignal = true;
			console.log(`Door state changed successfully.`);
		} else {
			testReport.doorSignal = false;
			console.log(`Door state did not change.`);
		}
		isQuestion.value = true;
	} catch (error) {
		console.error(`Error testing door sensor:`, error);
	}
}

const testStatus = computed(() => {
	if (!testCompleted.value) return "Test in progress...";

	const ledTestsPassed = Object.values(testReport.leds).every(
		(led) => led.on && led.backup
	);
	const dipSwitchesPassed = Object.values(testReport.dipSwitches).every(
		Boolean
	);
	const ctrlSignalsPassed = Object.values(testReport.ctrlSignal).every(
		Boolean
	);
	const doorSensorPassed = testReport.doorSignal;

	testPassed.value = {
		leds: ledTestsPassed,
		dipSwitches: dipSwitchesPassed,
		ctrlSignals: ctrlSignalsPassed,
		doorSensor: doorSensorPassed,
	};

	if (Object.values(testPassed.value).every(Boolean)) {
		return "All tests passed! 🎉";
	}

	return "Some tests failed. ❌";
});

async function startTest() {
	try {
		logs.value = [];
		testCompleted.value = false;
		await sendCommand(DeviceCommands.SELECT_DEVICE + "0");

		const response = await sendCommand(DeviceCommands.READ_STATUS);
		const report = parseDeviceReport(response);
		testReport.gdSnr = report.gdSnr;
		GDSnrInTest.value = testReport.gdSnr.slice(1);

		// LEDs Test
		for (const color of Object.keys(LedColors) as Array<
			keyof typeof LedColors
		>) {
			await testLedColors(color);
			await testBackupLeds(color);
			await sendCommand("L0");
		}
		testPassed.value.leds = Object.values(testReport.leds).every(
			(led) => led.on && led.backup
		);

		// Dip Switches Test
		for (const sw of Object.keys(testReport.dipSwitches).map(Number)) {
			await testDeepSwitch(sw);
		}
		testPassed.value.dipSwitches = Object.values(
			testReport.dipSwitches
		).every(Boolean);

		// Control Signals Test
		for (const sw of Object.keys(testReport.ctrlSignal).map(Number)) {
			await testCtrlSignal(sw);
		}
		testPassed.value.ctrlSignals = Object.values(
			testReport.ctrlSignal
		).every(Boolean);

		// Door Sensor Test
		await testDoorSensor();
		testPassed.value.doorSensor = testReport.doorSignal;

		await sendCommand(DeviceCommands.SET_LED_STATE + 0);
		testCompleted.value = true;
		
		if (testPassed.value) await printLabel(testReport.gdSnr);
	} catch (error) {
		console.error("Error during test:", error);
	}
}

async function printLabel(gdSnr:string) {
	try {
		const response = await fetch("https://top-dane-clearly.ngrok-free.app/print", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ text: gdSnr }), 
		});

		if (!response.ok) {
			throw new Error(`Failed to print label: ${response.statusText}`);
		}

		const data = await response.json();
		console.log("Label printed successfully:", data);
	} catch (error) {
		console.error("Error printing sticker:", error);
	}
}

async function emergencyStop() {
	await sendCommand("L0");
	logs.value = [];

	logContainer.value = null;

	feedbackVisible.value = false;
	feedbackQuestion.value = "";
	feedbackCallback.value = null;
	isQuestion.value = true;

	GDSnrInTest.value = "";

	testPassed.value = {
		leds: true,
		dipSwitches: true,
		ctrlSignals: true,
		doorSensor: true,
	};

	testCompleted.value = false;
}

/* ---- Watcher ---- */
watch(
	logs,
	() => {
		setTimeout(() => {
			logContainer.value?.scrollTo({
				top: logContainer.value.scrollHeight,
				behavior: "smooth",
			});
		}, 50);
	},
	{ deep: true }
);
</script>
