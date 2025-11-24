<script>
    import ApplicationForm from "$lib/components/forms/ApplicationForm.svelte";
    export let params;
</script>

<ApplicationForm applicationID={Number(params.id)} />
