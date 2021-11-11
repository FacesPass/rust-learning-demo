import('./livemd').then((livemd) => {
  let editor = document.getElementById("editor");
  let preview = document.getElementById("preview");
  let markdownToHtml = () => {
    let markdownText = editor.value;
    html = livemd.parse(markdownText);
    preview.innerHTML = html;
  };
  editor.addEventListener('input', markdownToHtml);
  // 开始解析初始文本
  markdownToHtml();
}).catch(console.error);