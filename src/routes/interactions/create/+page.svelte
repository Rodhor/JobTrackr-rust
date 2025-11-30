<script lang="ts">
    import { page } from "$app/state";
    import InteractionForm from "$lib/components/forms/InteractionForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");
    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];
</script>

<InteractionForm {callerChain} />
