<script lang="ts">
    import { page } from "$app/stores";
    import CompanyForm from "$lib/components/forms/CompanyForm.svelte";

    const caller = $page.url.searchParams.get("caller");
</script>

<CompanyForm {caller} />
