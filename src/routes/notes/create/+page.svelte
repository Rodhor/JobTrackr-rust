<script lang="ts">
    import { page } from "$app/stores";
    import NoteForm from "$lib/components/forms/NoteForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<NoteForm {caller} />
