<script lang="ts">
    import PersonForm from "$lib/components/forms/PersonForm.svelte";
    export let params;
</script>

<PersonForm personID={Number(params.id)} />
