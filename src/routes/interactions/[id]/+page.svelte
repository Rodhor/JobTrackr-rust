<script lang="ts">
    import InteractionForm from "$lib/components/forms/InteractionForm.svelte";
    export let params;
</script>

<InteractionForm interactionID={Number(params.id)} />
