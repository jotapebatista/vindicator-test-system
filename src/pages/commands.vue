<template>
	<LayoutTile
		title="USB RS232 Devices"
		description="Search and view available USB RS232 devices connected to the system."
	>
		<div class="space-y-6">
			<!-- Search Button -->
			<div flex items-center justify-between>
				<button
					class="rounded-md bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
					@click="fetchUsbDevices"
				>
					Search Devices:
				</button>
			</div>

			<!-- Devices List -->
			<div v-if="devices.length" class="space-y-4">
				<h3 class="text-xl text-white font-semibold">
					Available Devices:
				</h3>
				<ul class="space-y-2">
					<li
						v-for="(device, index) in devices"
						:key="index"
						class="flex flex-row items-center justify-around gap-4 rounded-lg bg-white/10 p-2 shadow-md"
					>
						<span class="text-sm text-gray-400">{{
							device.path
						}}</span>
						<button
							class="rounded-md bg-green-500 px-4 py-2 text-white hover:bg-green-600"
							@click="connectDevice(device)"
						>
							Connect
						</button>
					</li>
				</ul>
			</div>

			<p v-else class="text-gray-500">
				No USB RS232 devices found.
			</p>
		</div>
	</LayoutTile>
</template>

<script lang="ts" setup>
	import { SerialPort } from "tauri-plugin-serialplugin"; // To navigate between pages
	import { ref } from "vue";
	import { useRouter } from "vue-router";

	// List of USB devices
	const devices = ref<{ name: string, path: string }[]>([]);

	// Create a reference for the Vue Router
	const router = useRouter();

	// Fetch available USB devices and filter tty devices
	const fetchUsbDevices = async () => {
		try {
			const allPorts = await SerialPort.available_ports();

			if (allPorts && typeof allPorts === "object") {
				const allPortsArray = Object.entries(allPorts)
					.map(([path, details]) => ({
						path,
						...details
					}))
					.filter((device) => device.path.startsWith("/dev/tty")); // Only tty devices

				devices.value = allPortsArray;
			} else {
				console.warn("Unexpected data format for ports:", allPorts);
			}
		} catch (error) {
			console.error("Failed to scan ports:", error);
		}
	};

	// Connect function: sends the selected device to another page
	const connectDevice = (device: { name: string, path: string }) => {
		// You can use query params or a state management solution, like Vuex, to pass the selected device
		// For simplicity, we use the route's query params here.
		router.push({
			name: "notifications", // Replace with the actual route name for the next page
			query: { path: device.path, name: device.name }
		});
	};
</script>
