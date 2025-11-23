<script>
    import CompanyForm from "$lib/components/forms/CompanyForm.svelte";
    export let params;
</script>

<CompanyForm companyID={Number(params.id)} />
