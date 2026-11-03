// Mermaid 11.16.0, one file from mermaid-compact 0.0.1.
// License: /static/js/mermaid.LICENSE.txt
import mermaid from "/static/js/mermaid-11.16.0.mjs";

const nodes = [...document.querySelectorAll(".mermaid")].map((el) => ({
  el,
  source: el.textContent.trim(),
}));

const themeConfig = () => {
  const css = getComputedStyle(document.documentElement);
  const pick = (name) => css.getPropertyValue(name).trim();
  const ink = pick("--ink");
  const paper = pick("--paper");
  const paper2 = pick("--paper-2");
  const accent = pick("--accent");
  const rule = pick("--rule");
  const font = getComputedStyle(document.body).fontFamily;
  return {
    startOnLoad: false,
    securityLevel: "strict",
    suppressErrorRendering: true,
    theme: "base",
    look: "classic",
    fontFamily: font,
    flowchart: { htmlLabels: false, curve: "step", padding: 12, useMaxWidth: true },
    themeVariables: {
      darkMode: document.documentElement.getAttribute("data-theme") === "dark",
      background: paper,
      fontFamily: font,
      fontSize: "16px",
      primaryColor: paper2,
      primaryTextColor: ink,
      primaryBorderColor: accent,
      secondaryColor: paper,
      secondaryTextColor: ink,
      secondaryBorderColor: rule,
      tertiaryColor: paper,
      tertiaryTextColor: ink,
      tertiaryBorderColor: rule,
      lineColor: accent,
      arrowheadColor: accent,
      textColor: ink,
      mainBkg: paper2,
      nodeBorder: accent,
      clusterBkg: paper,
      clusterBorder: rule,
      titleColor: ink,
      edgeLabelBackground: paper,
      nodeTextColor: ink,
      actorBkg: paper2,
      actorBorder: accent,
      actorTextColor: ink,
      actorLineColor: accent,
      signalColor: ink,
      signalTextColor: ink,
      labelBoxBkgColor: paper2,
      labelBoxBorderColor: accent,
      labelTextColor: ink,
      loopTextColor: ink,
      noteBkgColor: paper2,
      noteTextColor: ink,
      noteBorderColor: rule,
      activationBorderColor: accent,
      activationBkgColor: paper2,
      sequenceNumberColor: paper,
    },
  };
};

let generation = 0;

const draw = async () => {
  const ticket = ++generation;
  mermaid.initialize(themeConfig());
  for (let i = 0; i < nodes.length; i += 1) {
    if (ticket !== generation) return;
    const { el, source } = nodes[i];
    const id = `mmd-${ticket}-${i}`;
    try {
      const { svg, bindFunctions } = await mermaid.render(id, source);
      if (ticket !== generation) return;
      el.innerHTML = svg;
      bindFunctions?.(el);
      el.dataset.rendered = "true";
      el.classList.remove("mermaid-error");
    } catch (error) {
      if (ticket !== generation) return;
      document.getElementById(id)?.remove();
      el.classList.add("mermaid-error");
      delete el.dataset.rendered;
      el.textContent = source;
      console.error(error);
    }
  }
};

if (nodes.length) {
  await draw();
  new MutationObserver(() => {
    draw();
  }).observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["data-theme"],
  });
}
