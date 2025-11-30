<script lang="ts">
    import { page } from "$app/state";
    import NoteForm from "$lib/components/forms/NoteForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");

    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];
</script>

<NoteForm {callerChain} />
