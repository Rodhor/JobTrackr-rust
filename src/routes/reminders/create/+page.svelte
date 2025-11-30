<script lang="ts">
    import { page } from "$app/state";
    import ReminderForm from "$lib/components/forms/ReminderForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain") || "";
    const callerChain = callerChainParam
        ? callerChainParam.split(",").filter(Boolean)
        : [];
</script>

<ReminderForm {callerChain} />
