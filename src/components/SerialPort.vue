<template>
	<div class="mx-auto max-w-4xl p-6">
		<!-- Title -->
		<h1 class="mb-6 text-center text-3xl text-gray-800 font-bold">
			Serial Port Communication
		</h1>

		<!-- Button to list available serial ports -->
		<div class="mb-6">
			<button
				class="rounded-md bg-blue-500 px-4 py-2 text-white transition hover:bg-blue-700"
				@click="listPorts"
			>
				List Available Ports
			</button>
		</div>

		<!-- Available Ports List -->
		<div v-if="ports.length" class="mb-6">
			<h2 class="mb-4 text-2xl text-gray-700 font-semibold">
				Available Ports
			</h2>
			<ul class="space-y-2">
				<li
					v-for="port in ports"
					:key="port"
					class="cursor-pointer rounded-md bg-gray-100 p-3 transition hover:bg-gray-200"
					@click="selectPort(port)"
				>
					{{ port }}
				</li>
			</ul>
		</div>

		<!-- Error Message Display -->
		<div
			v-if="errorMessage"
			class="mb-6 rounded-md bg-red-100 p-4 text-red-800"
		>
			<h2 class="font-bold">
				Error
			</h2>
			<p>{{ errorMessage }}</p>
		</div>

		<!-- Connection Buttons -->
		<div class="mb-6 flex justify-between">
			<button
				:disabled="isConnected"
				class="rounded-md bg-green-500 px-4 py-2 text-white transition hover:bg-green-700 disabled:opacity-50"
				@click="connectToSerialPort"
			>
				Connect to Serial Port
			</button>
			<button
				:disabled="!isConnected"
				class="rounded-md bg-yellow-500 px-4 py-2 text-white transition hover:bg-yellow-700 disabled:opacity-50"
				@click="sendData"
			>
				Send Command
			</button>
			<button
				:disabled="!isConnected"
				class="rounded-md bg-blue-500 px-4 py-2 text-white transition hover:bg-blue-700 disabled:opacity-50"
				@click="readData"
			>
				Read Data
			</button>
		</div>

		<!-- Data Received -->
		<div v-if="dataReceived" class="mb-6">
			<h2 class="mb-2 text-2xl text-gray-700 font-semibold">
				Received Data
			</h2>
			<pre
				class="rounded-md bg-gray-200 p-4 text-sm text-gray-800 font-mono"
			>{{ dataReceived }}</pre>
		</div>

		<!-- Log Window -->
		<div v-if="logs.length" class="mt-6">
			<h2 class="mb-4 text-2xl text-gray-700 font-semibold">
				Log
			</h2>
			<div
				class="max-h-60 overflow-auto rounded-md bg-gray-100 p-4 text-sm font-mono"
				style="height: 300px;"
			>
				<ul class="space-y-2">
					<li
						v-for="(log, index) in logs"
						:key="index"
						class="border-b border-gray-300 pb-2"
					>
						<span :class="log.type === 'sent' ? 'text-blue-500' : 'text-green-500'">
							{{ log.type === 'sent' ? 'Sent: ' : 'Received: ' }}
						</span>
						{{ log.message }}
					</li>
				</ul>
			</div>
		</div>
	</div>
</template>

<script setup>
	import { invoke } from "@tauri-apps/api/core";
	import { ref } from "vue";

	const ports = ref([]);
	const dataReceived = ref(null);
	const errorMessage = ref(null);
	const portName = ref("");
	const isConnected = ref(false);
	const logs = ref([]); // Store the logs

	// List available serial ports
	const listPorts = async () => {
		try {
			const availablePorts = await invoke("list_ports");
			ports.value = availablePorts;
		} catch (error) {
			console.error("Error listing ports:", error);
			errorMessage.value = `Error listing ports: ${error.message}`;
		}
	};

	// Select a serial port
	const selectPort = (port) => {
		portName.value = port;
		errorMessage.value = null;
	};

	// Connect to the selected serial port
	const connectToSerialPort = async () => {
		if (!portName.value) {
			errorMessage.value = "Please select a port.";
			return;
		}

		try {
			await invoke("open_port", {
				portName: portName.value
			});
			isConnected.value = true;
			errorMessage.value = null;
			logs.value.push({ type: "sent", message: `Connected to ${portName.value}` });
		} catch (error) {
			isConnected.value = false;
			errorMessage.value = `Error opening port: ${error.message}`;
		}
	};

	// Read data from the serial port
	const readData = async () => {
		if (!isConnected.value) {
			errorMessage.value = "No connection established.";
			return;
		}

		try {
			const data = await invoke("read_from_port", {
				portName: portName.value
			});
			dataReceived.value = data;
			logs.value.push({ type: "received", message: data });
		} catch (error) {
			errorMessage.value = `Error reading data: ${error.message}`;
		}
	};

	// Send command to the serial port
	const sendData = async () => {
		if (!isConnected.value) {
			errorMessage.value = "No connection established.";
			return;
		}

		try {
			const command = "S0\r\n";
			await invoke("write_to_port", {
				portName: portName.value,
				data: command
			});
			logs.value.push({ type: "sent", message: `Sent: ${command}` });

			// Read the response after sending the command
			await readData();

			const command2 = "F0\r\n";
			await invoke("write_to_port", {
				portName: portName.value,
				data: command2
			});
			logs.value.push({ type: "sent", message: `Sent: ${command2}` });

			await readData();

			const command3 = "R\r\n";
			await invoke("write_to_port", {
				portName: portName.value,
				data: command3
			});
			logs.value.push({ type: "sent", message: `Sent: ${command3}` });

			await readData();

			const command4 = "C02,00,00\r\n";
			await invoke("write_to_port", {
				portName: portName.value,
				data: command4
			});
			logs.value.push({ type: "sent", message: `Sent: ${command4}` });

			await readData();

			const command5 = "FA\r\n";
			await invoke("write_to_port", {
				portName: portName.value,
				data: command5
			});
			logs.value.push({ type: "sent", message: `Sent: ${command5}` });

			await readData();
		} catch (error) {
			errorMessage.value = `Error sending data: ${error.message}`;
		}
	};
</script>

<style scoped>
	/* TailwindCSS styling is applied within the template above */
</style>
