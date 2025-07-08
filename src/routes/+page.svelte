<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Calendar from "$lib/components/ui/calendar/calendar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    import "../index.css";
    import type { PackageJson } from "./types";

    let path = $state("");
    let packageName = $state("");
    let version = $state("");
    let greetMsg = $state("");
    let folderPath = $state<string | null>("");

    let packageInfo = $state<PackageJson>();

    async function openPath() {
        const openFolder = open({
            directory: true,
            filters: [
                {
                    name: "folders Only",
                    extensions: ["directory"],
                },
            ],
        });
        folderPath = await openFolder;
        console.log(folderPath);
    }

    async function readPath(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        invoke("find_package_json", { name: path })
            .then((message) => console.log(message))
            .catch((value) => {
                console.error(value);
            });
    }
</script>

<main class="container">
    <div class="bg-gray-900">
        <div class="flex flex-col justify-center items-center gap-y-4">
            <form class="flex flex-row" onsubmit={readPath}>
                <input
                    id="path-input"
                    placeholder="Enter a path..."
                    bind:value={path}
                />
                <Button type="submit">find</Button>
            </form>

            <form onsubmit={openPath}>
                <Button variant="default" type="submit">Open file</Button>
            </form>
        </div>
        <div>
            <p>{packageInfo?.name}</p>
            <p>{packageInfo?.version}</p>
            <p>{packageInfo?.description}</p>
        </div>
    </div>
</main>

<style>
    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        margin: 0;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    input,
    input,
    button {
        outline: none;
    }

    #path-input {
        margin-right: 5px;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }

        input,
        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
        button:active {
            background-color: #0f0f0f69;
        }
    }
</style>
