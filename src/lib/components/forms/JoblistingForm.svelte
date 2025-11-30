<script lang="ts">
    import { goto } from "$app/navigation";
    import CustomIDSelectCreate from "./utils/CustomIDSelectCreate.svelte";
    import { Button } from "$lib/components/ui/button";
    import { companies } from "$lib/stores/companies";
    import { newlyCreatedJobListingId } from "$lib/stores/formState";
    import {
        createJobListing,
        jobListings,
        updateJobListing,
    } from "$lib/stores/jobListings";
    import { Currency, SeniorityLevel, WorkType } from "$lib/types/enums";
    import type { JobListing } from "$lib/types/jobListing";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";

    let {
        joblistingID,
        callerChain = [],
    }: { joblistingID?: number; callerChain?: string[] } = $props();

    const items = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name,
            })),
    );

    let form = $state<JobListing>({
        companyId: undefined,
        title: "",
        workType: WorkType.Other,
        category: "",
        seniorityLevel: SeniorityLevel.Mid,
        salaryMin: undefined,
        salaryMax: undefined,
        currency: Currency.EUR,
        description: "",
        url: "",
    });

    let companyId = $state<number | undefined>(undefined);

    $effect(() => {
        if (!joblistingID) return;
        const found = $jobListings.find((j) => j.id === joblistingID);
        if (found) {
            Object.assign(form, found);
            if (found.companyId) {
                companyId = found.companyId;
            }
        }
    });

    $effect(() => {
        form.companyId = companyId;
    });

    const invalidSubmit = $derived(
        companyId === undefined || !form.title.trim(),
    );

    async function submit() {
        if (invalidSubmit) return;

        let createdListing: JobListing;

        if (joblistingID) {
            await updateJobListing(Number(joblistingID), form);
            createdListing = form;
        } else {
            createdListing = await createJobListing(form);
        }

        console.log("JobListing created:", $state.snapshot(form));
        console.log("CallerChain:", callerChain);

        if (callerChain.length > 0 && !joblistingID) {
            newlyCreatedJobListingId.set(createdListing.id);
        }

        // Go back to where we came from
        if (callerChain.length > 0) {
            // Get the page we came from (last item in chain)
            const backToCaller = callerChain[callerChain.length - 1];
            // Build new chain WITHOUT the current page (remove last item)
            const newChain = callerChain.slice(0, -1);

            if (newChain.length > 0) {
                // Pass remaining chain to previous page
                const params = new URLSearchParams();
                params.set("callerChain", newChain.join(","));
                const url = `/${backToCaller}/create?  ${params.toString()}`;
                console.log(
                    "Navigating back to:",
                    url,
                    "with chain:",
                    newChain,
                );
                await goto(url);
            } else {
                // No more chain, just go back to caller
                const url = `/${backToCaller}/create`;
                console.log("Navigating back to:", url, "no chain");
                await goto(url);
            }
        } else {
            console.log("No chain, going to /jobListings");
            await goto("/jobListings");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            const url = `/${callerChain[0]}/create`;
            console.log("Cancel - Going to:", url);
            goto(url);
        } else {
            console.log("Cancel - Going to /jobListings");
            goto("/jobListings");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {joblistingID ? "Edit Job Listing" : "Create a new Job Listing"}
    </h1>

    <div class="space-y-4">
        <div>
            <Label for="company" class="py-2 required">Company</Label>
            <CustomIDSelectCreate
                {items}
                bind:value={companyId}
                {callerChain}
                currentPage="jobListings"
                createNew="companies"
            />
        </div>

        <div>
            <Label for="title" class="py-2 required">Title</Label>
            <Input
                id="title"
                bind:value={form.title}
                placeholder="Position title"
            />
        </div>

        <div>
            <Label for="workType" class="py-2">Work Type</Label>
            <CustomEnumSelector
                enumObject={WorkType}
                bind:selectedValue={form.workType}
            />
        </div>

        <div>
            <Label for="seniorityLevel" class="py-2">Seniority Level</Label>
            <CustomEnumSelector
                enumObject={SeniorityLevel}
                bind:selectedValue={form.seniorityLevel}
            />
        </div>

        <div>
            <Label for="currency" class="py-2">Currency</Label>
            <CustomEnumSelector
                enumObject={Currency}
                bind:selectedValue={form.currency}
            />
        </div>

        <div>
            <Label for="salaryMin" class="py-2">Min Salary</Label>
            <Input
                id="salaryMin"
                type="number"
                bind:value={form.salaryMin}
                placeholder="50000"
            />
        </div>

        <div>
            <Label for="salaryMax" class="py-2">Max Salary</Label>
            <Input
                id="salaryMax"
                type="number"
                bind:value={form.salaryMax}
                placeholder="70000"
            />
        </div>

        <div>
            <Label for="description" class="py-2">Description</Label>
            <Textarea
                id="description"
                bind:value={form.description}
                placeholder="Job description..."
            />
        </div>

        <div>
            <Label for="url" class="py-2">Application URL</Label>
            <Input
                id="url"
                bind:value={form.url}
                placeholder="https://example.   com/job"
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {joblistingID ? "Update" : "Create"}
        </Button>
    </div>
</section>
