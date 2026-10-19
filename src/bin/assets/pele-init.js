// Initialize Pele for mdBook diagrams.
// Compatible with pre.mermaid elements produced by the preprocessor.
(function () {
  function init() {
    if (typeof pele === "undefined" || typeof pele.mount !== "function") {
      console.error("Pele is not loaded");
      return;
    }
    document.querySelectorAll("pre.mermaid").forEach(function (el) {
      // Skip if already mounted
      if (el.dataset.peleMounted) return;
      el.dataset.peleMounted = "true";
      try {
        pele.mount(el, el.textContent);
      } catch (e) {
        console.error("Pele failed to render diagram:", e);
      }
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
