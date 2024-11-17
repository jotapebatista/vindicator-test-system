<template>
	<div relative overflow-hidden px-6 lg="px-8">
		<div grid mx-auto h-screen max-w-2xl place-content-center>
			<SvgoLogo
				:filled="true"
				:font-controlled="false"
				mx-auto
				mb-8
				size-40
			/>

			<div text-center>
				<h1
					animate-pulse
					text-4xl
					text-light-300
					font-bold
					tracking-wider
					font-heading
					sm="text-6xl"
				>
					{{ name.toUpperCase() }}
				</h1>

				<div mt-15>
					<NuxtLink
						gap-1
						rounded-md
						bg-emerald-500
						px-3.5
						py-2.5
						text-sm
						text-white
						font-semibold
						shadow-sm
						transition-colors
						hover="bg-emerald-600"
						focus-visible="outline-2 outline-emerald-600 outline-offset-2 outline"
						to="/select-port"
					>
						Start Test
					</NuxtLink>
				</div>
			</div>
			<!-- <SerialPort /> -->
		</div>

		<div fixed bottom-6 text-sm absolute-center-h>
			<p text-sm text-neutral-500>
				Made by {{ author }}
			</p>
		</div>
	</div>
</template>

<script lang="ts" setup>
	import { useRouter } from "vue-router";

	const { name, author } = useConstants();
	const router = useRouter();

	definePageMeta({
		layout: "home"
	});

	const startTest = () => {
		router.push({
			name: "start-test"
		});
	};
</script>
