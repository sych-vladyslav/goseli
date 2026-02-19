'use client';

import { formatPrice } from '@/lib/api';

interface CartSummaryProps {
  subtotal: number;
  itemCount: number;
  onCheckout?: () => void;
}

export function CartSummary({ subtotal, itemCount, onCheckout }: CartSummaryProps) {
  const shipping = 0;
  const tax = 0;
  const total = subtotal + shipping + tax;

  return (
    <div className="card sticky top-20">
      <div className="card-body">
        <h2 className="text-lg font-bold text-neutral-900 mb-4">Order Summary</h2>

        <div className="space-y-3 mb-4">
          <div className="flex items-center justify-between text-sm">
            <span className="text-neutral-600">
              Subtotal ({itemCount} {itemCount === 1 ? 'item' : 'items'})
            </span>
            <span className="font-medium text-neutral-900">{formatPrice(subtotal)}</span>
          </div>

          <div className="flex items-center justify-between text-sm">
            <span className="text-neutral-600">Shipping</span>
            <span className="font-medium text-neutral-900">
              {shipping === 0 ? 'FREE' : formatPrice(shipping)}
            </span>
          </div>

          <div className="flex items-center justify-between text-sm">
            <span className="text-neutral-600">Tax</span>
            <span className="font-medium text-neutral-900">
              {tax === 0 ? 'Calculated at checkout' : formatPrice(tax)}
            </span>
          </div>
        </div>

        <div className="border-t border-neutral-200 pt-4 mb-6">
          <div className="flex items-center justify-between">
            <span className="text-lg font-bold text-neutral-900">Total</span>
            <span className="text-xl font-bold text-neutral-900">{formatPrice(total)}</span>
          </div>
        </div>

        <button
          onClick={onCheckout}
          className="btn btn-primary btn-lg w-full"
          aria-label="Proceed to checkout"
        >
          Proceed to Checkout
        </button>

        <p className="text-xs text-neutral-500 text-center mt-4">
          Shipping and taxes calculated at checkout
        </p>
      </div>
    </div>
  );
}
