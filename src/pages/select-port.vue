<template>
	<LayoutTile
		title="Select Device"
		description="Select the device you want to connect to."
	>
		<div class="p-4 space-y-4">
			<!-- <h2 class="text-xl text-gray-800 font-semibold dark:text-gray-200">
				Available USB Devices
			</h2> -->
			<div
				v-if="devices.length > 0"
				class="grid grid-cols-1 gap-4 md:grid-cols-2"
			>
				<div
					v-for="device in devices"
					:key="device.path"
					class="flex items-center justify-between border border-gray-200 rounded-lg bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800"
				>
					<div>
						<p
							class="text-sm text-gray-900 font-medium dark:text-gray-100"
						>
							{{ device.name || "Unknown Device" }}
						</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">
							Path: {{ device.path }}
						</p>
					</div>
					<button
						class="rounded-lg bg-blue-600 px-4 py-2 text-sm text-white font-medium shadow hover:bg-blue-700 focus:ring focus:ring-blue-300 dark:focus:ring-blue-500"
						@click="connectDevice(device)"
					>
						Connect
					</button>
				</div>
			</div>
			<p v-else class="text-sm text-gray-600 dark:text-gray-400">
				No USB devices found. Ensure a device is connected and try
				again.
			</p>
		</div>
	</LayoutTile>
</template>

<script lang="ts" setup>
	import { invoke } from "@tauri-apps/api/core";
	import { onMounted, ref } from "vue";
	import { useRouter } from "vue-router";

	const devices = ref<{ name: string, path: string }[]>([]);
	const router = useRouter();

	const getPortPrefix = () => {
		const os = navigator.userAgent;
		if (os.includes("Windows")) {
			return "COM";
		} else if (os.includes("Macintosh") || os.includes("Mac OS X")) {
			return "/dev/cu";
		} else if (os.includes("Linux")) {
			return "/dev/tty";
		}
		return "";
	};

	const fetchUsbDevices = async () => {
		try {
			const portPrefix = getPortPrefix();
			const allPorts = await invoke("list_ports");

			if (allPorts && Array.isArray(allPorts)) {
				devices.value = allPorts
					.map((port: string) => ({
						name: `Device ${port}`, // Placeholder for device name
						path: port
					}))
					.filter((device) => device.path.startsWith(portPrefix));
			} else {
				console.warn("Unexpected data format for ports:", allPorts);
			}
		} catch (error) {
			console.error("Failed to scan ports:", error);
		}
	};

	const connectDevice = (device: string) => {
		console.log(device);
		router.push({
			name: "test",
			query: device
		});
	};

	onMounted(() => {
		fetchUsbDevices();
	});
</script>
