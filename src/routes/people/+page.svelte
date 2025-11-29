<script lang="ts">
    import { onMount } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { people, loadPeople, deletePerson } from "$lib/stores/people";
    import { Role, RoleDisplay } from "$lib/types/enums";

    const roleColorMap: Record<Role, string> = {
        recruiter: "bg-blue-100 text-blue-800",
        hiring_manager: "bg-green-100 text-green-800",
        team_lead: "bg-yellow-100 text-yellow-800",
        developer: "bg-purple-100 text-purple-800",
        hr: "bg-pink-100 text-pink-800",
        founder: "bg-red-100 text-red-800",
        other: "bg-muted text-foreground",
    };

    onMount(loadPeople);

    function getRoleColor(role?: Role): string {
        return role ? roleColorMap[role] : roleColorMap[Role.Other];
    }

    async function handleDelete(id: number) {
        await deletePerson(id);
    }
</script>

<div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
        <h1 class="text-2xl font-semibold tracking-tight">People</h1>
        <Button href="/people/create">New Person</Button>
    </div>

    <!-- Table -->
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="min-w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
            >
                <tr>
                    <th class="px-4 py-3">Name</th>
                    <th class="px-4 py-3">Email</th>
                    <th class="px-4 py-3">Phone</th>
                    <th class="px-4 py-3">Role</th>
                    <th class="px-4 py-3">LinkedIn</th>
                    <th class="px-4 py-3 text-right">Actions</th>
                </tr>
            </thead>

            <tbody>
                {#each $people as p (p.id)}
                    <tr class="border-t transition-colors hover:bg-muted/30">
                        <td class="px-4 py-3 font-medium">
                            {p.firstName}
                            {p.lastName}
                        </td>
                        <td class="px-4 py-3 text-muted-foreground">
                            {p.email || "—"}
                        </td>
                        <td class="px-4 py-3 text-muted-foreground">
                            {p.phoneNumber || "—"}
                        </td>
                        <td class="px-4 py-3">
                            <Badge class={getRoleColor(p.role)}>
                                {p.role ? RoleDisplay[p.role] : "—"}
                            </Badge>
                        </td>
                        <td
                            class="px-4 py-3 truncate max-w-[200px] text-muted-foreground"
                        >
                            {#if p.linkedinUrl}
                                <a
                                    href={p.linkedinUrl}
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {p.linkedinUrl}
                                </a>
                            {:else}
                                —
                            {/if}
                        </td>
                        <td class="px-4 py-3 text-right">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    href="/people/{p.id}"
                                >
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    onclick={() => p.id && handleDelete(p.id)}
                                >
                                    Delete
                                </Button>
                            </div>
                        </td>
                    </tr>
                {/each}

                {#if $people.length === 0}
                    <tr>
                        <td
                            colspan="6"
                            class="px-4 py-10 text-center text-sm text-muted-foreground"
                        >
                            No people yet.
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>
