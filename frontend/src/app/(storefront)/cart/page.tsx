'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useCart } from '@/lib/hooks';
import { CartItemRow } from '@/components/CartItemRow';
import { CartSummary } from '@/components/CartSummary';

export default function CartPage() {
  const router = useRouter();
  const { cart, isLoading, updateCartItem, removeCartItem, clearCart } = useCart();

  const handleUpdateQuantity = async (itemId: string, quantity: number) => {
    await updateCartItem(itemId, { quantity });
  };

  const handleRemoveItem = async (itemId: string) => {
    await removeCartItem(itemId);
  };

  const handleClearCart = async () => {
    if (window.confirm('Are you sure you want to clear your cart?')) {
      await clearCart();
    }
  };

  const handleCheckout = () => {
    router.push('/checkout');
  };

  if (isLoading) {
    return (
      <div className="container-custom py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="flex flex-col items-center gap-4">
            <svg className="animate-spin h-8 w-8 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" aria-hidden="true">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <p className="text-neutral-600">Loading cart...</p>
          </div>
        </div>
      </div>
    );
  }

  const isEmpty = !cart || cart.items.length === 0;

  if (isEmpty) {
    return (
      <div className="container-custom py-8">
        <h1 className="text-3xl font-bold text-neutral-900 mb-8">Shopping Cart</h1>
        <div className="flex flex-col items-center justify-center min-h-[400px] card card-body-lg">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-24 w-24 text-neutral-300 mb-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1}
              d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
            />
          </svg>
          <h2 className="text-xl font-semibold text-neutral-900 mb-2">Your cart is empty</h2>
          <p className="text-neutral-600 mb-6 text-center">
            Looks like you have not added anything to your cart yet.
          </p>
          <Link href="/products" className="btn btn-primary btn-lg">
            Continue Shopping
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="container-custom py-8">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-3xl font-bold text-neutral-900">Shopping Cart</h1>
        <button
          onClick={handleClearCart}
          className="btn btn-ghost btn-sm text-error hover:text-error-dark"
          aria-label="Clear cart"
        >
          Clear Cart
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2">
          <div className="card">
            <div className="card-body">
              <div className="divide-y divide-neutral-200">
                {cart.items.map((item) => (
                  <CartItemRow
                    key={item.id}
                    item={item}
                    onUpdateQuantity={handleUpdateQuantity}
                    onRemove={handleRemoveItem}
                  />
                ))}
              </div>
            </div>
          </div>

          <div className="mt-4 flex items-center justify-between">
            <Link href="/products" className="btn btn-outline btn-md">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fillRule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clipRule="evenodd" />
              </svg>
              Continue Shopping
            </Link>
          </div>
        </div>

        <div className="lg:col-span-1">
          <CartSummary
            subtotal={cart.total}
            itemCount={cart.item_count}
            onCheckout={handleCheckout}
          />
        </div>
      </div>
    </div>
  );
}
