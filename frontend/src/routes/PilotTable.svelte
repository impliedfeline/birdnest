<script>
	import Pilot from './Pilot.svelte';

	export let pilots;
</script>

<table>
	<tr>
		<th>Name</th>
		<th>Phone number</th>
		<th>Email</th>
		<th>Closest confirmed distance to nest</th>
	</tr>
	{#each pilots as pilot (pilot.id)}
		<Pilot {...pilot} />
	{/each}
</table>
