<template>
	<div v-if="visible" class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
		<div class="rounded-lg bg-slate-700 p-6 shadow-lg">
			<h3 class="mb-4 text-lg font-semibold">
				{{ question }}
			</h3>
			<div class="flex justify-between gap-4">
				<!-- Conditional Buttons -->
				<template v-if="confirmBtn">
					<button class="rounded bg-green-500 px-4 py-2 text-white hover:bg-green-600" @click="onRespond(true)">
						Yes
					</button>
					<button class="rounded bg-red-500 px-4 py-2 text-white hover:bg-red-600" @click="onRespond(false)">
						No
					</button>
				</template>
				<template v-else>
					<button class="rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600" @click="onRespond(true)">
						Continue
					</button>
				</template>
			</div>
		</div>
	</div>
</template>

<script lang="ts" setup>
import { defineEmits, defineProps } from "vue";

defineProps({
	visible: Boolean,
	question: String,
	confirmBtn: Boolean,
});

const emit = defineEmits(["respond"]);

function onRespond(response: boolean) {
	emit("respond", response);
}
</script>
