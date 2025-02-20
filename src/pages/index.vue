<template>
	<div relative overflow-hidden px-6 lg="px-8">
		<div grid mx-auto h-screen max-w-2xl place-content-center>
			<!-- <SvgoLogo
				:filled="true"
				:font-controlled="false"
				mx-auto
				mb-8
				size-40
			/> -->

			<img src="~/assets/vindi.png" alt="" />
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

				<div mt-15 flex justify-center mb-8>
					<NuxtLink
						to="/select-port"
						class="inline-flex items-center rounded-md bg-emerald-500 px-3.5 py-2.5 text-sm font-semibold text-white transition-colors duration-300 hover:bg-emerald-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-emerald-600"
					>
						<Icon name="ph:play" size="20" class="mr-2" />
						Start Test
					</NuxtLink>
				</div>
			</div>
			<!-- <SerialPort /> -->
		</div>

		<div fixed bottom-6 text-sm absolute-center-h>
			<p text-sm text-neutral-500>
				Made by
				<span
					class="hover:underline hover:cursor-pointer hover:text-white transition-colors duration-500"
					@click="onClick"
					target="_blank"
				>
					{{ author }}
				</span>
			</p>
		</div>
	</div>
</template>

<script lang="ts" setup>
const { name, author, repo } = useConstants();
import { open } from "@tauri-apps/plugin-shell";
definePageMeta({
	layout: "home",
});

const onClick = () => {
	open(repo);
};
</script>
