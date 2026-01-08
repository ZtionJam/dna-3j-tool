#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
从 icon.png 生成所有需要的图标文件
"""
import os
import sys
from PIL import Image

# 设置输出编码为 UTF-8（Windows 控制台支持）
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

# 图标目录
ICONS_DIR = "src-tauri/icons"
SOURCE_ICON = os.path.join(ICONS_DIR, "icon.png")

# 需要生成的图标尺寸列表
ICON_SIZES = {
    # PNG 文件
    "128x128.png": 128,
    "128x128@2x.png": 256,  # 2x 表示双倍分辨率
    "32x32.png": 32,
    "Square107x107Logo.png": 107,
    "Square142x142Logo.png": 142,
    "Square150x150Logo.png": 150,
    "Square284x284Logo.png": 284,
    "Square30x30Logo.png": 30,
    "Square310x310Logo.png": 310,
    "Square44x44Logo.png": 44,
    "Square71x71Logo.png": 71,
    "Square89x89Logo.png": 89,
    "StoreLogo.png": 50,  # Windows Store 通常使用 50x50
}

# ICO 文件需要的尺寸（Windows 图标需要多个尺寸）
ICO_SIZES = [16, 32, 48, 64, 128, 256]


def resize_image(input_path, output_path, size, quality=95):
    """调整图片大小并保存"""
    try:
        img = Image.open(input_path)
        # 使用高质量的重采样算法
        resized = img.resize((size, size), Image.Resampling.LANCZOS)
        resized.save(output_path, "PNG", optimize=True)
        print(f"[OK] 生成 {output_path} ({size}x{size})")
        return True
    except Exception as e:
        print(f"[ERROR] 生成 {output_path} 失败: {e}")
        return False


def create_ico(input_path, output_path, sizes):
    """创建 ICO 文件（Windows 图标）"""
    try:
        images = []
        for size in sizes:
            img = Image.open(input_path)
            resized = img.resize((size, size), Image.Resampling.LANCZOS)
            images.append(resized)
        
        # 保存为 ICO 格式
        images[0].save(
            output_path,
            format="ICO",
            sizes=[(s, s) for s in sizes],
            append_images=images[1:] if len(images) > 1 else []
        )
        print(f"[OK] 生成 {output_path} (包含尺寸: {', '.join(map(str, sizes))})")
        return True
    except Exception as e:
        print(f"[ERROR] 生成 {output_path} 失败: {e}")
        return False


def main():
    """主函数"""
    print("开始生成图标文件...")
    print(f"源文件: {SOURCE_ICON}")
    print("-" * 50)
    
    # 检查源文件是否存在
    if not os.path.exists(SOURCE_ICON):
        print(f"错误: 找不到源文件 {SOURCE_ICON}")
        return
    
    # 生成 PNG 文件
    print("\n生成 PNG 文件:")
    for filename, size in ICON_SIZES.items():
        output_path = os.path.join(ICONS_DIR, filename)
        resize_image(SOURCE_ICON, output_path, size)
    
    # 生成 ICO 文件
    print("\n生成 ICO 文件:")
    ico_path = os.path.join(ICONS_DIR, "icon.ico")
    create_ico(SOURCE_ICON, ico_path, ICO_SIZES)
    
    print("\n" + "-" * 50)
    print("图标生成完成！")
    print(f"注意: icon.icns 文件需要 macOS 工具生成，已跳过")


if __name__ == "__main__":
    main()

