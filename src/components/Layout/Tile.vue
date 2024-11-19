<template>
	<div class="flex flex-col items-center lg:flex-row lg:items-start space-y-6 lg:space-x-12 lg:space-y-0">
		<!-- Left Section -->
		<div class="mx-auto max-w-xl px-6 pb-10 pt-12 lg:mx-0 lg:px-8 lg:pt-12">
			<h2 class="text-3xl text-white font-bold tracking-tight">
				{{ props.title }}
			</h2>
			<p class="mt-6 text-lg text-gray-300 leading-8">
				{{ props.description }}
			</p>
		</div>

		<!-- Right Section (Slot for dynamic content) -->
		<div class="w-full px-6 pb-10 pt-12 lg:px-8 lg:pt-22 sm:pt-16">
			<slot />
		</div>
	</div>
</template>

  <script lang="ts" setup>
	const props = defineProps<{
		title: string
		description: string
	}>();
  </script>
