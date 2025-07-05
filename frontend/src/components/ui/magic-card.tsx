import { cn } from "@/lib/utils";
import { motion } from "framer-motion";
import { ReactNode } from "react";

interface MagicCardProps {
  children: ReactNode;
  className?: string;
  gradientSize?: number;
  gradientColor?: string;
  gradientOpacity?: number;
}

export const MagicCard = ({
  children,
  className,
  gradientSize = 200,
  gradientColor = "#262626",
  gradientOpacity = 0.8,
}: MagicCardProps) => {
  return (
    <motion.div
      className={cn(
        "group relative h-full w-full overflow-hidden rounded-xl border border-gray-200 bg-white p-6 shadow-sm transition-all duration-300 hover:shadow-lg dark:border-gray-800 dark:bg-gray-900",
        className
      )}
      whileHover={{ y: -2 }}
      transition={{ type: "spring", stiffness: 300, damping: 30 }}
      style={{
        background: `radial-gradient(${gradientSize}px circle at var(--mouse-x) var(--mouse-y), ${gradientColor}${Math.round(
          gradientOpacity * 255
        ).toString(16)}, transparent 40%)`,
      }}
      onMouseMove={(e) => {
        const rect = e.currentTarget.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        e.currentTarget.style.setProperty("--mouse-x", `${x}px`);
        e.currentTarget.style.setProperty("--mouse-y", `${y}px`);
      }}
    >
      <div className="relative z-10">{children}</div>
      
      {/* Animated border gradient */}
      <div className="absolute inset-0 rounded-xl opacity-0 transition-opacity duration-300 group-hover:opacity-100">
        <div className="absolute inset-0 rounded-xl bg-gradient-to-r from-blue-500/20 via-purple-500/20 to-pink-500/20 blur-sm" />
      </div>
    </motion.div>
  );
}; 