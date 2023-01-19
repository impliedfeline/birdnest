<script>
	import PilotTable from './PilotTable.svelte';

	async function getPilots() {
		const response = await fetch('/api/pilots');
		const pilots = await response.json();

		if (response.ok) {
			return pilots.map(handlePilotInfo);
		} else {
			throw new Error(text);
		}
	}

	function handlePilotInfo(info) {
		const pilot = {};
		pilot.id = info.id;
		pilot.name = info.pilot.firstName + ' ' + info.pilot.lastName;
		pilot.phoneNumber = info.pilot.phoneNumber;
		pilot.email = info.pilot.email;
		pilot.distance = info.distance;
		return pilot;
	}

	let promise = getPilots();

	setInterval(() => {
		promise = getPilots();
	}, 10000);
</script>

{#await promise then pilots}
	<PilotTable {pilots} />
{:catch error}
	<p>{error.message}</p>
{/await}
