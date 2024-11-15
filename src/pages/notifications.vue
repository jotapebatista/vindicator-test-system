<template>
	<LayoutTile
		title="Send Command to Device"
		description="Send commands to the connected USB RS232 device."
	>
		<form @submit.prevent="sendCommandToDevice">
			<div class="mx-auto max-w-xl lg:mr-0 lg:max-w-lg">
				<div class="grid grid-cols-1 gap-x-8 gap-y-6">
					<!-- Device Info Display -->
					<div>
						<label
							for="device-info"
							block
							text-sm
							text-white
							font-semibold
							leading-6
						>Connected Device</label>
						<div mt="2.5">
							<p class="text-white">
								<strong>Name:</strong> {{ deviceName }} <br>
								<strong>Path:</strong> {{ devicePath }}
							</p>
						</div>
					</div>

					<!-- Command Input -->
					<div>
						<label
							for="command-input"
							block
							text-sm
							text-white
							font-semibold
							leading-6
						>Send Command</label>
						<div mt="2.5">
							<input
								id="command-input"
								v-model="command"
								type="text"
								name="command"
								block
								w-full
								border-0
								rounded-md
								bg="white/5"
								px="3.5"
								py-2
								text-white
								shadow-sm
								ring-1
								ring="white/10"
								ring-inset
								sm="text-sm leading-6"
								focus="ring-2 ring-emerald-500 ring-inset"
							>
						</div>
					</div>

					<!-- Submit Button -->
					<div class="flex justify-end">
						<Btn type="submit">
							Send Command
						</Btn>
					</div>

					<!-- Error Message -->
					<p
						v-if="commandError"
						mt-3
						text-right
						text-sm
						text-red-500
						font-semibold
						leading-6
					>
						Failed to send command
					</p>

					<!-- Device Response Log -->
					<div class="mt-4">
						<h3 class="text-lg text-white font-semibold">
							Device Responses:
						</h3>
						<div
							class="h-48 overflow-auto border rounded-md bg-gray-800 p-4 text-white"
							style="white-space: pre-wrap; word-wrap: break-word"
						>
							<div
								v-for="(response, index) in responseLog"
								:key="index"
							>
								{{ response }}
							</div>
						</div>
					</div>
				</div>
			</div>
		</form>
	</LayoutTile>
</template>

<script lang="ts" setup>
	import { SerialPort } from "tauri-plugin-serialplugin";
	import { ref, watchEffect } from "vue"; // To access the query params
	import { useRoute } from "vue-router";

	// Retrieve the device details from the query params
	const route = useRoute();
	const devicePath = ref(route.query.path as string);
	const deviceName = ref(route.query.name as string);
	const command = ref("S0\r\nR\r\n");
	const commandError = ref(false);
	const responseLog = ref<string[]>([]); // Holds all device responses

	async function sendCommandToDevice() {
		let responseText = "";

		try {
			// Close port if already open
			if (port.isOpen) {
				console.log("Port is already open. Closing and reopening...");
				await closePort(port);
			}

			// Open the port
			await port.open();
			console.log("Port opened successfully");

			// Start listening for incoming data (before sending any commands)
			await port.startListening();

			// Send initial commands
			console.log("Sending select, setColor, flashLeds...");
			await port.write(messages.value.select);
			await port.write(messages.value.setColor);
			await port.write(messages.value.flashLeds);

			// Wait for the response and save the message
			responseText = await new Promise((resolve) => {
				port.listen((data) => {
					const decodedData = new TextDecoder().decode(data);
					console.log("Received:", decodedData);
					responseText += decodedData;

					// Check if the expected response is received
					if (decodedData.includes("expected response text")) {
						resolve(responseText); // Resolve the promise when the expected text is found
					}
				});
			});

			console.log("Final response text:", responseText);
		} catch (error) {
			console.error("Error during connection:", error);
			connectionStatus.value = "Disconnected";
			connectionStatusMessage.value
				= "Error occurred while trying to connect.";
		} finally {
			// Ensure the port is closed after the operation
			if (port.isOpen) {
				console.log("Closing port...");
				await port.write(messages.value.stop); // Stop any ongoing operations
				await port.stopListening(); // Stop listening for further data
				await closePort(port); // Close the port
			}
		}
	}
	// const sendCommandToDevice = async () => {
	// 	let responseText = "";
	// 	if (!devicePath.value || !command.value) {
	// 		commandError.value = true;
	// 		return;
	// 	}

	// 	try {
	// 		const port = new SerialPort({
	// 			path: devicePath.value,
	// 			baudRate: 9600
	// 		});

	// 		await port.open();

	// 		// Send the command
	// 		await port.write(command.value);

	// 		responseText = await new Promise((resolve, reject) => {
	// 			port.listen((data) => {
	// 				console.log("listen", data);
	// 				const decodedData = new TextDecoder().decode(data);
	// 				console.log("Received:", decodedData);
	// 				responseText += decodedData;

	// 				if (decodedData.includes("expected response text")) {
	// 					resolve(responseText);
	// 				}
	// 			});
	// 		});

	// 		// console.log(responseText);

	// 		// Listen for responses
	// 		await port.listen((data) => {
	// 			const responseString
	// 				= typeof data === "string"
	// 					? data
	// 					: new TextDecoder().decode(data);

	// 			console.log("Received from device:", responseString);
	// 			responseLog.value.push(responseString); // Add the response to the log
	// 		});

	// 		// Wait for some time to listen for responses (adjust as needed)
	// 		setTimeout(async () => {
	// 			await port.close(); // Close the port after listening for a while
	// 			command.value = "";
	// 			commandError.value = false;
	// 		}, 2000); // Adjust the timeout duration based on how long you expect the response to come
	// 	} catch (error) {
	// 		console.error("Failed to send command:", error);
	// 		commandError.value = true;
	// 	}
	// };

	// Watch for changes in the route (in case the user navigates directly to this page with different query params)
	watchEffect(() => {
		devicePath.value = route.query.path as string;
		deviceName.value = route.query.name as string;
	});
</script>
