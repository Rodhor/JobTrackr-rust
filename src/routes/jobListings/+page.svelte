<script lang="ts">
    import { jobListings, deleteJobListing } from "$lib/stores/jobListings";
    import { companies } from "$lib/stores/companies";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import {
        WorkType,
        WorkTypeDisplay,
        SeniorityLevelDisplay,
    } from "$lib/types/enums";
    import Trash2Icon from "lucide-svelte/icons/trash-2";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import BriefcaseIcon from "lucide-svelte/icons/briefcase";
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import DollarSignIcon from "lucide-svelte/icons/dollar-sign";

    async function handleDelete(id: number) {
        await deleteJobListing(id);
    }

    function formatDate(dateStr: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function getCompanyName(companyId?: number) {
        if (!companyId) return "—";
        return $companies.find((c) => c.id === companyId)?.name ?? "—";
    }

    const typeColorMap: Record<WorkType, string> = {
        [WorkType.Remote]: "bg-blue-100 text-blue-800",
        [WorkType.InOffice]: "bg-green-100 text-green-800",
        [WorkType.Hybrid]: "bg-yellow-100 text-yellow-800",
        [WorkType.Other]: "bg-gray-100 text-gray-800",
        [WorkType.FullTime]: "bg-indigo-100 text-indigo-800",
        [WorkType.PartTime]: "bg-cyan-100 text-cyan-800",
        [WorkType.Internship]: "bg-amber-100 text-amber-800",
        [WorkType.Contract]: "bg-rose-100 text-rose-800",
        [WorkType.Freelance]: "bg-teal-100 text-teal-800",
    };

    function typeColor(type?: WorkType): string {
        return type ? typeColorMap[type] : "bg-muted text-foreground";
    }

    function formatSalary(
        salaryMin?: number,
        salaryMax?: number,
        currency?: string,
    ): string {
        if (!salaryMin || !salaryMax) return "—";
        return `${salaryMin.toLocaleString()}–${salaryMax.toLocaleString()} ${currency || ""}`.trim();
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Job Listings</h1>
        <p class="text-muted-foreground mt-1">
            Manage and track job opportunities
        </p>
    </div>
    <Button href="/jobListings/create" size="lg">+ New Listing</Button>
</div>

<!-- ----------------------------------------------------------
Stats Card
----------------------------------------------------------- -->
<div class="mb-8 rounded-lg border border-border bg-background p-4">
    <div class="flex items-center justify-between">
        <div>
            <div class="text-sm font-medium text-muted-foreground mb-1">
                Total Listings
            </div>
            <div class="text-3xl font-bold">{$jobListings.length}</div>
        </div>
        <BriefcaseIcon class="size-12 text-muted-foreground opacity-30" />
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $jobListings.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <BriefcaseIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No job listings yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start tracking job opportunities
        </p>
        <Button href="/jobListings/create">Create First Listing</Button>
    </div>
{:else}
    <!-- ----------------------------------------------------------
    Table
    ----------------------------------------------------------- -->
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground border-b border-border"
            >
                <tr>
                    <th class="px-6 py-4 font-semibold">Title</th>
                    <th class="px-6 py-4 font-semibold">Company</th>
                    <th class="px-6 py-4 font-semibold">Work Type</th>
                    <th class="px-6 py-4 font-semibold">Seniority</th>
                    <th class="px-6 py-4 font-semibold">Salary</th>
                    <th class="px-6 py-4 font-semibold">Created</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>

            <tbody>
                {#each $jobListings as listing (listing.id)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Title -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {listing.title}
                            </div>
                        </td>

                        <!-- Company -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {getCompanyName(listing.companyId)}
                        </td>

                        <!-- Work Type -->
                        <td class="px-6 py-4">
                            <Badge
                                class={typeColor(listing.workType as WorkType)}
                            >
                                {WorkTypeDisplay[
                                    listing.workType as WorkType
                                ] || "—"}
                            </Badge>
                        </td>

                        <!-- Seniority -->
                        <td class="px-6 py-4">
                            {#if listing.seniorityLevel}
                                <span class="text-foreground"
                                    >{SeniorityLevelDisplay[
                                        listing.seniorityLevel
                                    ]}</span
                                >
                            {:else}
                                <span class="text-muted-foreground">—</span>
                            {/if}
                        </td>

                        <!-- Salary -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <DollarSignIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {formatSalary(
                                    listing.salaryMin,
                                    listing.salaryMax,
                                    listing.currency,
                                )}
                            </div>
                        </td>

                        <!-- Created Date -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {listing.createdAt
                                    ? formatDate(listing.createdAt)
                                    : "—"}
                            </div>
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/jobListings/{listing.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    class="gap-2"
                                    onclick={() =>
                                        listing.id && handleDelete(listing.id)}
                                >
                                    <Trash2Icon class="size-4" />
                                    Delete
                                </Button>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
