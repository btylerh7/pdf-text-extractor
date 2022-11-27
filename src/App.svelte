<script lang="ts">
  import {open, save} from '@tauri-apps/api/dialog'
  import { invoke } from '@tauri-apps/api/tauri'
  
  let currentText: String = ""
  const readFileContents = async () => {
    const selectedPath = await open(
      {
        multiple: false,
        title: "Select pdf"
      }
    )
    const text:String = await invoke('extract_text', {ogFilepath: selectedPath})
    return currentText = text
  }
  const saveAsText = async () => {
    const selectedPath = await save({
      filters: [{
        name: "TextFile",
        extensions: ['txt']
    }]
  })
    return await invoke('save_as_text', {newFilepath: selectedPath, text: currentText})
  }
</script>
<div class="container">
  <h1>The Ultimate converter app</h1>
  <ol>
    <li>Choose a file</li>
    <li>Format your text</li>
    <li>Copy and paste in quizlet or save as <code>.txt</code> file</li>
  </ol>
  <!-- <input type="file" name="pdf" id="pdf" bind:value={file}> -->
  <div class="input">
    <button on:click={readFileContents} type="button">Choose File</button>
    {#if currentText != ""}
    <textarea name="edit" id="edit" cols="30" rows="10" bind:value={currentText}></textarea>
    <button on:click={saveAsText} type="button">Save As</button>
    {/if}
  </div>
  
</div>



<style>
  .container {
    padding: 1rem 3rem;
  }
  .input {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    justify-content: center;
    align-items: center;
  }
  button {
    max-width: fit-content;
  }
  ol {
    display: flex;
    justify-content: center;
    gap: 5rem;
  }
  textarea {
    font-size: 20px;
    width: 500px;
    height: 500px;
  }
</style>