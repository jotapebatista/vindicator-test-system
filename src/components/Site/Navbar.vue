<template>
	<header top-0 z-10>
		<nav flex items-center justify-between py-6 crate>

			<div flex lg="absolute top-4 left-4 z-10">
			<NuxtLink to="/" class="home" p-1.5 m="-1.5">
				<img src="~/assets/barix_white.png" alt="" class="w-32" />
			</NuxtLink>
		</div>
			
		</nav>
	</header>
</template>


<style scoped>
	.router-link-exact-active:not(.home){
		@apply text-emerald-500;
	}
</style>
