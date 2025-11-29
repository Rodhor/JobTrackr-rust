<script lang="ts">
    import NoteForm from "$lib/components/forms/NoteForm.svelte";
    export let params;
</script>

<NoteForm noteID={Number(params.id)} />
