<template>
	<div>
		<SiteNavbar sticky />
		<SiteSidebar />

		<div crate>
			<DesignTopBlob />
			<DesignBottomBlob />
			<slot />
		</div>
	</div>
</template>
