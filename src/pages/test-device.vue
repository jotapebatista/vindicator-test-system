<template>
	<LayoutTile
		title="Vindicator Test Wizard"
		description="Follow the steps to test the device."
	>
		<div class="relative mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
			<!-- Always visible buttons -->
			<div class="mb-4">
				<button
					class="rounded-md bg-blue-600 px-4 py-2 text-sm text-white font-semibold hover:bg-blue-700"
					@click="startTest"
				>
					Start Test
				</button>
			</div>

			<!-- Message Logs Section -->
			<div class="mb-4">
				<div
					class="h-64 overflow-y-auto border border-gray-200 shadow-sm rounded-lg bg-gray-900 p-4 text-wrap"
					ref="logContainer"
				>
					<div
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
							<p class="text-white text-sm">
								{{ message.text }}
							</p>
						</div>
					</div>
				</div>
			</div>

			<FeedbackModal
				v-if="feedbackVisible"
				:visible="feedbackVisible"
				:question="feedbackQuestion"
				@respond="handleFeedback"
			/>

			<!-- Fixed buttons -->
			<div class="fixed bottom-4 right-4 z-50 space-x-2">
				<button
					class="rounded-md bg-blue-600 px-4 py-2 text-sm text-white font-semibold hover:bg-blue-700"
					@click="restartTest"
				>
					Restart Test
				</button>
				<button
					class="rounded-md bg-red-600 px-4 py-2 text-sm text-white font-semibold hover:bg-red-700"
					@click="quitTest"
				>
					Quit Test
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
  RED: "Cff,00,00",
  GREEN: "C00,ff,00",
  BLUE: "C00,00,ff",
};

const route = useRoute();
const devicePath = ref(route.query.path as string);
const logs = ref<{ type: string; text: string }[]>([]);
const logContainer = ref<HTMLDivElement | null>(null);
const feedbackVisible = ref(false);
const feedbackQuestion = ref("");
const feedbackCallback = ref<((response: boolean) => void) | null>(null);

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
      baseReadings: { current, inputVoltage, capacitorVoltage, temperature: degTemperature },
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
	await sendCommand("L1")
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
	await sendCommand("L1")
	await sendCommand("P1")
    const response = await showFeedback(`Is the Backup ${color} LED on?`);

    testReport.leds[color] = {
      ...testReport.leds[color],
      backup: response,
    };
	await sendCommand("P0")
  } catch (error) {
    console.error(`Error testing ${color} LED:`, error);
  }
}

async function flashLedColors(color: keyof typeof LedColors) {
	try {
    await sendCommand(LedColors[color]);
	await sendCommand("FA")
    const response = await showFeedback(`Is the flash ${color} LED flashing?`);

	testReport.leds[color] = {
      ...testReport.leds[color],
      flash: response,
    };

  } catch (error) {
    console.error(`Error testing ${color} LED:`, error);
  }
}


async function startTest() {
  try {
    await sendCommand(DeviceCommands.SELECT_DEVICE + "0");
    const response = await sendCommand(DeviceCommands.READ_STATUS);
    const report = parseDeviceReport(response);
	testReport.gdSnr = report.gdSnr

    for (const color of Object.keys(LedColors) as Array<keyof typeof LedColors>) {
      await testLedColors(color);
	  await testBackupLeds(color)
	  await sendCommand("L0")
    }

	for (const color of Object.keys(LedColors) as Array<keyof typeof LedColors>) {
      await flashLedColors(color);
    }

	for (const sw in Object.keys(testReport.dipSwitches) as Array<keyof typeof testReport.dipSwitches>) {
		await sendCommand(DeviceCommands.READ_STATUS);
	}


    await sendCommand(DeviceCommands.SET_LED_STATE + 0); // Reset
	console.log(testReport)
  } catch (error) {
    console.error("Error during test:", error);
  }
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
