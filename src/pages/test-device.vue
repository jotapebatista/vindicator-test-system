<template>
	<LayoutTile
		title="Vindicator Test Wizard"
		description="Follow the steps to test the device."
	>
		<div class="relative mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
			<!-- Always visible buttons -->
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
import { invoke } from "@tauri-apps/api/core";
import { computed, onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
const devicePath = ref(route.query.path as string);
const testingCurrentLeds = ref(false);

const testReport = {
	gdSnr: "",
	leds:{
		red: {
			on: false,
			flash: false,
			backup: false,
			current: 0,
		},
		green: {
			on: false,
			flash: false,
			backup: false,
			current: 0,
		},
		blue: {
			on: false,
			flash: false,
			backup: false,
			current: 0,
		},
	deepSwitches:{
		1: false,
		2: false,
		3: false,
		4: false,
		5: false
	},
	ctrlSignal: {
		1: false,
		2: false
	}

	}
}

async function getDeviceReport() {
	try {
		return await invoke("write_to_port", {
			path: devicePath.value,
			data: `R\r\n`,
		});
	} catch (error) {
		console.error("Error sending command:", error);
		throw error;
	}
}

async function switchLeds(command) {
	try {
		const res = await invoke("write_to_port", {
			path: devicePath.value,
			data: `${command}\r\n`,
		});
		return res;
	} catch (error) {
		console.error("Error sending command:", error);
		throw error;
	}
}

async function testLedColors(colors: string) {
	try {
		const res = await invoke("write_to_port", {
			path: devicePath.value,
			data: `${colors}\r\n`,
		});
		return res;
	} catch (error) {
		console.error("Error sending command:", error);
		throw error;
	}
}

async function switchCurrentLeds() {
	try {
		const res = await invoke("write_to_port", {
			path: devicePath.value,
			data: `${testingCurrentLeds.value ? "1" : "0"}\r\n`,
		});
		return res;
	} catch (error) {
		console.error("Error sending command:", error);
		throw error;
	}
}
</script>
