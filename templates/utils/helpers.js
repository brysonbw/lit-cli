/**
 * @param {unknown} input
 * @returns {boolean}
 */
function isBool(input) {
  return typeof input === "boolean";
}

/**
 * @param {unknown} input
 * @returns {boolean}
 */
function isNum(input) {
  if (typeof input === "number") {
    return !isNaN(input);
  }

  if (typeof input === "string") {
    const parsed = parseFloat(input);
    return !isNaN(parsed);
  }

  return false;
}

/**
 * @param {unknown} input
 * @returns  {boolean}
 */
function isPlainObject(input) {
  if (typeof input !== "object" || input === null) return false;

  const proto = Object.getPrototypeOf(input);

  // Handles Objects created with Object.create(null)
  if (proto === null) return true;

  // Ensures the constructor is the global Object function
  const Ctor =
    Object.prototype.hasOwnProperty.call(proto, "constructor") &&
    proto.constructor;
  return typeof Ctor === "function" && Ctor === Object;
}

/**
 *  @param {unknown} input
 *  @returns {string}
 */
function toSentenceCase(input) {
  const strClean =
    typeof input === "string" && input.length > 0 ? input.trim() : null;
  if (!strClean) {
    return "";
  }

  const withSpaces = strClean.replace(/([A-Z])/g, " $1");

  return withSpaces.charAt(0).toUpperCase() + withSpaces.slice(1).toLowerCase();
}

/**
 * @param {unknown} input
 * @returns {string}
 */
function toTitleCase(input) {
  const strClean =
    typeof input === "string" && input.length > 0 ? input.trim() : null;
  if (!strClean) {
    return "";
  }

  const withSpaces = strClean.replace(/([A-Z])/g, " $1");

  return withSpaces
    .split(/\s+/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}

export { isBool, isNum, isPlainObject, toSentenceCase, toTitleCase };
