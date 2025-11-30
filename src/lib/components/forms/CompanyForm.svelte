<script lang="ts">
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import { WorkType } from "$lib/types/enums";
    import type { Company } from "$lib/types/company";
    import { Button } from "$lib/components/ui/button";
    import { goto } from "$app/navigation";
    import {
        updateCompany,
        createCompany,
        companies,
    } from "$lib/stores/companies";
    import { newlyCreatedCompanyId } from "$lib/stores/formState";

    let {
        companyID,
        callerChain = [],
    }: { companyID?: number; callerChain?: string[] } = $props();

    let form = $state<Company>({
        name: "",
        industry: "",
        streetAddress: "",
        zipCode: "",
        country: "",
        defaultWorkType: WorkType.Other,
        website: "",
        phoneNumber: "",
    });

    $effect(() => {
        if (!companyID) return;
        const found = $companies.find((c) => c.id === companyID);
        if (found) Object.assign(form, found);
    });

    const invalidSubmit = $derived(!form.name);

    async function submit() {
        if (invalidSubmit) return;

        let createdCompany: Company;

        if (companyID) {
            await updateCompany(Number(companyID), form);
            createdCompany = form;
        } else {
            createdCompany = await createCompany(form);
        }

        if (callerChain.length > 0 && !companyID) {
            newlyCreatedCompanyId.set(createdCompany.id);
        }

        if (callerChain.length > 0) {
            // Get the page we came from (last item in chain)
            const backToCaller = callerChain[callerChain.length - 1];

            // Build new chain WITHOUT the current page (remove last item)
            const newChain = callerChain.slice(0, -1);

            if (newChain.length > 0) {
                // Pass remaining chain to previous page
                const params = new URLSearchParams();
                params.set("callerChain", newChain.join(","));
                const url = `/${backToCaller}/create?${params.toString()}`;
                await goto(url);
            } else {
                // No more chain, just go back to caller
                const url = `/${backToCaller}/create`;
                await goto(url);
            }
        } else {
            await goto("/companies");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            const url = `/${callerChain[0]}/create`;
            goto(url);
        } else {
            goto("/companies");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {companyID ? "Edit Company" : "Create a new Company"}
    </h1>

    <div class="space-y-4">
        <div>
            <Label for="companyName" class="py-2 required">Company Name</Label>
            <Input
                id="companyName"
                bind:value={form.name}
                placeholder="Enter company name"
            />
        </div>

        <div>
            <Label for="industry" class="py-2">Industry</Label>
            <Input
                id="industry"
                bind:value={form.industry}
                placeholder="Software / Socialwork"
            />
        </div>

        <div>
            <Label for="streetAddress" class="py-2">Street Address</Label>
            <Input
                id="streetAddress"
                bind:value={form.streetAddress}
                placeholder="Mainstreet 6"
            />
        </div>

        <div>
            <Label for="zipCode" class="py-2">Zip Code</Label>
            <Input id="zipCode" bind:value={form.zipCode} placeholder="12345" />
        </div>

        <div>
            <Label for="country" class="py-2">Country</Label>
            <Input
                id="country"
                bind:value={form.country}
                placeholder="Germany"
            />
        </div>

        <div>
            <Label for="worktype" class="py-2">Default Work Type</Label>
            <CustomEnumSelector
                enumObject={WorkType}
                bind:selectedValue={
                    form.defaultWorkType as WorkType | undefined
                }
            />
        </div>

        <div>
            <Label for="website" class="py-2">Website</Label>
            <Input
                id="website"
                bind:value={form.website}
                placeholder="www.google.com"
            />
        </div>

        <div>
            <Label for="phone" class="py-2">Phone Number</Label>
            <Input
                id="phone"
                bind:value={form.phoneNumber}
                placeholder="+49 0123456789"
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {companyID ? "Update" : "Create"}
        </Button>
    </div>
</section>
