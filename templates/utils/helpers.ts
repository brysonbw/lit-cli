function isBool(input: unknown): boolean {
  return typeof input === "boolean";
}

function isNum(input: unknown): boolean {
  if (typeof input === "number") {
    return !isNaN(input);
  }

  if (typeof input === "string") {
    const parsed = parseFloat(input);
    return !isNaN(parsed);
  }

  return false;
}

function isPlainObject(input: unknown): boolean {
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

function toSentenceCase(input: unknown): string {
  const strClean =
    typeof input === "string" && input.length > 0 ? input.trim() : null;
  if (!strClean) {
    return "";
  }

  const withSpaces = strClean.replace(/([A-Z])/g, " $1");

  return withSpaces.charAt(0).toUpperCase() + withSpaces.slice(1).toLowerCase();
}

function toTitleCase(input: unknown): string {
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
