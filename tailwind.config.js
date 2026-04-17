/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#007AFF',
          light: '#E1F0FF',
          dark: '#0051A8',
        },
        success: '#34C759',
        warning: '#FF9500',
        danger: '#FF3B30',
        up: '#FF3B30',
        down: '#34C759',
        neutral: {
          50: '#FFFFFF',
          100: '#F8F9FA',
          200: '#E9ECEF',
          300: '#ADB5BD',
          400: '#495057',
          500: '#212529',
        }
      },
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', '"SF Pro"', 'Inter', 'sans-serif'],
      },
      fontSize: {
        'display-lg': ['32px', { lineHeight: '1.2', fontWeight: '700' }],
        'heading-1': ['24px', { lineHeight: '1.3', fontWeight: '600' }],
        'heading-2': ['20px', { lineHeight: '1.3', fontWeight: '600' }],
        'heading-3': ['16px', { lineHeight: '1.4', fontWeight: '600' }],
        'body': ['14px', { lineHeight: '1.5', fontWeight: '400' }],
        'caption': ['12px', { lineHeight: '1.5', fontWeight: '400' }],
        'tiny': ['10px', { lineHeight: '1.5', fontWeight: '400' }],
      },
      borderRadius: {
        'card': '12px',
        'modal': '16px',
        'button': '8px',
        'input': '8px',
        'tag': '6px',
      },
      boxShadow: {
        'card': '0 1px 2px rgba(0, 0, 0, 0.05)',
        'default': '0 4px 12px rgba(0, 0, 0, 0.08)',
        'hover': '0 8px 24px rgba(0, 0, 0, 0.12)',
        'modal': '0 12px 48px rgba(0, 0, 0, 0.16)',
      },
      animation: {
        'spring': 'spring 0.3s ease-out',
        'fade-in': 'fadeIn 0.25s ease-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'slide-down': 'slideDown 0.3s ease-out',
        'scale-in': 'scaleIn 0.3s ease-out',
        'pulse-soft': 'pulseSoft 2s ease-in-out infinite',
      },
      keyframes: {
        spring: {
          '0%': { transform: 'scale(0.95)' },
          '100%': { transform: 'scale(1)' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.9)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        pulseSoft: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.7' },
        },
      },
      transitionTimingFunction: {
        'spring': 'cubic-bezier(0.175, 0.885, 0.32, 1.275)',
      }
    },
  },
  plugins: [],
}
