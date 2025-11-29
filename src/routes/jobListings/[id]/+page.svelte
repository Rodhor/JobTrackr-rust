<script>
    import JobListingForm from "$lib/components/forms/JoblistingForm.svelte";
    export let params;
</script>

<JobListingForm joblistingID={Number(params.id)} />
