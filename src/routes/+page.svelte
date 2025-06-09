<script lang="ts">
	import githubFilled from '$lib/assets/icons/githubFilled.svg';
	import linkedinFilled from '$lib/assets/icons/linkedinFilled.svg';
	import instagramFilled from '$lib/assets/icons/instagramFilled.svg';
	import githubOutlined from '$lib/assets/icons/githubOutlined.svg';
	import linkedinOutlined from '$lib/assets/icons/linkedinOutlined.svg';
	import instagramOutlined from '$lib/assets/icons/instagramOutlined.svg';
	import mailOutlined from '$lib/assets/icons/mailOutlined.svg';
	import mailFilled from '$lib/assets/icons/mailFilled.svg';
</script>

<h2><span>About me</span></h2>

<h3><span>Projects</span></h3>

<h4><span>This website</span></h4>
<p>Font is the <a href="https://plus.excalidraw.com/excalifont">font used for Excalidraw</a></p>

<h4><span>Extended essay</span></h4>
<p>
	3500-word essay I wrote for high school evaluating the outsourcing of a SOC rather than social
	engineering training for a small business.
	<a href="/ee.pdf">Read it here</a>
</p>

<h3><span>Links</span></h3>
<div class="flex flex-wrap gap-4">
	<a href="https://github.com/alex-k-exe/" class="image-hover-container">
		<img src={githubOutlined} class="base-image" alt="Outlined Github logo" />
		<img src={githubFilled} class="hover-image" alt="Filled in Github logo" />
	</a>
	<a href="https://www.linkedin.com/in/alexkammin/" class="image-hover-container">
		<img src={linkedinOutlined} class="base-image" alt="Outlined Linkedin logo" />
		<img src={linkedinFilled} class="hover-image" alt="Filled in Linkedin logo" />
	</a>
	<a href="https://www.instagram.com/alex_k_exe/" class="image-hover-container">
		<img src={instagramOutlined} class="base-image" alt="Outlined Instagram logo" />
		<img src={instagramFilled} class="hover-image" alt="Filled in Instagram logo" />
	</a>
	<a href="mailto:alexkammin@outlook.com" class="image-hover-container">
		<img src={mailOutlined} class="base-image" alt="Outlined mail icon" />
		<img src={mailFilled} class="hover-image" alt="Filled in mail icon" />
	</a>
</div>

<style>
	.image-hover-container {
		position: relative;
		width: 3rem;
		height: 3rem;
		overflow: hidden;
	}

	.image-hover-container img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.hover-image {
		opacity: 0;
		pointer-events: none;
	}

	.image-hover-container:hover .hover-image {
		opacity: 1;
		pointer-events: auto;
	}

	.image-hover-container:hover .base-image {
		opacity: 0;
	}
</style>
