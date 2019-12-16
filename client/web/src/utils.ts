export function debounce(func: Function, wait = 1000) {
  let timeout: number = 0;
  function debounced(...args: any[]) {
    clearTimeout(timeout);
    timeout = window.setTimeout(() => {
      console.log('applying func with args:', args);
      func.apply(this, args);
    }, wait);
  };
  debounced.cancel = () => clearTimeout(timeout);
  return debounced
}

