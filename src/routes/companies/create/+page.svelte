<script lang="ts">
    import { page } from "$app/state";
    import CompanyForm from "$lib/components/forms/CompanyForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");
    console.log("Route: callerChainParam raw:", callerChainParam);

    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];

    console.log("Route: parsed callerChain:", callerChain);
</script>

<CompanyForm {callerChain} />
