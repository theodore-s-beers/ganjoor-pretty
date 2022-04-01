function fixup() {
  //
  // LINE NUMBER & SEPARATOR DIVS
  //

  // Select all first hemistichs
  const firstHems = document.querySelectorAll("div.m1");

  for (const element of firstHems) {
    // Add a div for line number before each first hemistich
    element.insertAdjacentHTML(
      "beforebegin",
      '<div class="line-number"></div>'
    );

    // Add a div for separator after each first hemistich
    element.insertAdjacentHTML("afterend", '<div class="separator"></div>');
  }

  //
  // FINAL LINE
  //

  // Select all whole lines
  const lines = document.querySelectorAll("div.b");

  // Select final line
  const finalLine = lines[lines.length - 1];

  // Apply special class to final line
  finalLine.classList.remove("b");
  finalLine.classList.add("final-line");

  //
  // ADD LINE NUMBERS
  //

  // Select all line number divs
  const lineNumbers = document.querySelectorAll("div.line-number");

  // Final line should never be numbered, so we stop iteration early
  for (let i = 0, n = lineNumbers.length; i < n - 1; i++) {
    // The actual line number, 1-indexed
    const count = i + 1;

    // If divisible by three, insert number
    if (count % 3 === 0) {
      // Number is also localized for Persian
      lineNumbers[i].innerHTML = `<p>${count.toLocaleString("fa-IR")}</p>`;
    }
  }

  //
  // HEMISTICH WIDTH
  //

  // I picked up this function from SO. It's supposed to use a canvas to
  // calculate the width of some text. We provide a string of text and the
  // desired font properties.
  function getTextWidth(text, font) {
    const canvas =
      getTextWidth.canvas ||
      (getTextWidth.canvas = document.createElement("canvas"));
    const context = canvas.getContext("2d");
    context.font = font;
    const metrics = context.measureText(text);
    return metrics.width;
  }

  // Select all p tags. This is an easy way of getting all hemistichs.
  const allHems = document.querySelectorAll("p");

  // Set up variable to hold max computed text width
  let maxWidth = 0;

  // Iterate over hemistichs, calculating width of each
  for (const element of allHems) {
    const testText = element.innerText;
    const hemWidth = getTextWidth(testText, "x-large Scheherazade");

    // If this hemistich's width is a new maximum, update
    if (hemWidth > maxWidth) {
      maxWidth = hemWidth;
    }
  }

  // Make sure we got an actual max width before doing anything else
  if (maxWidth !== 0) {
    // Round up maxWidth, and make it into a string with a px value for CSS
    const pixelWidth = Math.ceil(maxWidth) + "px";

    // Finally, update CSS custom property
    document.documentElement.style.setProperty("--hemistich-width", pixelWidth);
  }
}

window.onload = () => {
  fixup();
};
