'use client';

import { useState } from 'react';

export function AddToCartButton() {
  const [clicked, setClicked] = useState(false);

  const handleClick = () => {
    setClicked(true);
    setTimeout(() => {
      setClicked(false);
    }, 1500);
  };

  return (
    <button
      onClick={handleClick}
      disabled={clicked}
      className="btn btn-primary btn-lg w-full sm:w-auto"
      aria-label={clicked ? 'Item added to cart' : 'Add product to cart'}
    >
      {clicked ? 'Added!' : 'Add to Cart'}
    </button>
  );
}
