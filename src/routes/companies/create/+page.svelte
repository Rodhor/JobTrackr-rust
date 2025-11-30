<script lang="ts">
    import { page } from "$app/state";
    import CompanyForm from "$lib/components/forms/CompanyForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");

    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];
</script>

<CompanyForm {callerChain} />
