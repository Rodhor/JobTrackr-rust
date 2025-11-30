<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { people, deletePerson } from "$lib/stores/people";
    import { Role, RoleDisplay } from "$lib/types/enums";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import UserIcon from "lucide-svelte/icons/user";
    import LinkedinIcon from "lucide-svelte/icons/linkedin";
    import DeleteAlert from "$lib/components/forms/utils/DeleteAlert.svelte";

    let confirmDelete = $state(false);
    let selectedPersonId: number | null = $state(null);
    let selectedPersonName: string = $state("");

    $effect(() => {
        if (confirmDelete && selectedPersonId) {
            handleDelete(selectedPersonId);
            confirmDelete = false;
            selectedPersonId = null;
            selectedPersonName = "";
        }
    });

    const roleColorMap: Record<Role, string> = {
        recruiter: "bg-blue-100 text-blue-800",
        hiring_manager: "bg-green-100 text-green-800",
        team_lead: "bg-yellow-100 text-yellow-800",
        developer: "bg-purple-100 text-purple-800",
        hr: "bg-pink-100 text-pink-800",
        founder: "bg-red-100 text-red-800",
        other: "bg-muted text-foreground",
    };

    function getRoleColor(role?: Role): string {
        return role ? roleColorMap[role] : roleColorMap[Role.Other];
    }

    async function handleDelete(id: number) {
        await deletePerson(id);
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">People</h1>
        <p class="text-muted-foreground mt-1">
            Manage your contacts and network
        </p>
    </div>
    <Button href="/people/create" size="lg">+ New Person</Button>
</div>

<!-- ----------------------------------------------------------
Stats Card
----------------------------------------------------------- -->
<div class="mb-8 rounded-lg border border-border bg-background p-4">
    <div class="flex items-center justify-between">
        <div>
            <div class="text-sm font-medium text-muted-foreground mb-1">
                Total People
            </div>
            <div class="text-3xl font-bold">{$people.length}</div>
        </div>
        <UserIcon class="size-12 text-muted-foreground opacity-30" />
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $people.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <UserIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No people yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start building your network by adding contacts
        </p>
        <Button href="/people/create">Add First Person</Button>
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
                    <th class="px-6 py-4 font-semibold">Name</th>
                    <th class="px-6 py-4 font-semibold">Email</th>
                    <th class="px-6 py-4 font-semibold">Phone</th>
                    <th class="px-6 py-4 font-semibold">Role</th>
                    <th class="px-6 py-4 font-semibold">LinkedIn</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>

            <tbody>
                {#each $people as person (person.id)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Name -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {person.firstName}
                                {person.lastName}
                            </div>
                        </td>

                        <!-- Email -->
                        <td class="px-6 py-4">
                            {#if person.email}
                                <a
                                    href="mailto:{person.email}"
                                    class="text-blue-600 hover:underline truncate block"
                                >
                                    {person.email}
                                </a>
                            {:else}
                                <span class="text-muted-foreground">—</span>
                            {/if}
                        </td>

                        <!-- Phone -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {#if person.phoneNumber}
                                <a
                                    href="tel:{person.phoneNumber}"
                                    class="text-blue-600 hover:underline"
                                >
                                    {person.phoneNumber}
                                </a>
                            {:else}
                                —
                            {/if}
                        </td>

                        <!-- Role -->
                        <td class="px-6 py-4">
                            <Badge class={getRoleColor(person.role)}>
                                {person.role ? RoleDisplay[person.role] : "—"}
                            </Badge>
                        </td>

                        <!-- LinkedIn -->
                        <td class="px-6 py-4">
                            {#if person.linkedinUrl}
                                <a
                                    href={person.linkedinUrl}
                                    class="inline-flex items-center gap-2 text-blue-600 hover:underline"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    title={person.linkedinUrl}
                                >
                                    <LinkedinIcon class="size-4" />
                                    <span class="truncate max-w-[150px]"
                                        >Profile</span
                                    >
                                </a>
                            {:else}
                                <span class="text-muted-foreground">—</span>
                            {/if}
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/people/{person.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <DeleteAlert
                                    objectText="'{person.firstName} {person.lastName}'"
                                    description="This will delete the person record."
                                    onDelete={async () => {
                                        await deletePerson(person.id!);
                                    }}
                                />
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
