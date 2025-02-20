<template>
	<div class="relative min-h-screen overflow-hidden">
		<!-- Logo on the left -->
		<!-- <div class="absolute top-4 left-4 ">

			<img src="~/assets/barix_white.png" alt="" class="w-32" />
			
		</div> -->

		<div flex lg="absolute top-4 left-4 z-10">
			<NuxtLink to="/" class="home" p-1.5 m="-1.5">
				<img src="~/assets/barix_white.png" alt="" class="w-32" />
			</NuxtLink>
		</div>

		<div relative overflow-hidden px-6 lg="px-8">
			<DesignTopBlob />
			<DesignBottomBlob />
			<slot />
		</div>
	</div>
</template>
