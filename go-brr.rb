require 'pathname'
require 'erb'
require 'fileutils'

def get_lib_template()
  %{<% for @style in @styles -%>
#[cfg(feature = "<%= @style %>")]
pub mod <%= @style %>;
<% end %>

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,
    #[prop_or_default]
    pub size: Option<i64>,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub fill: Option<&'static str>,
    #[prop_or_default]
    pub stroke_width: Option<i64>,
    #[prop_or_default]
    pub stroke_linecap: Option<&'static str>,
    #[prop_or_default]
    pub stroke_linejoin: Option<&'static str>,
}
  }
end

def get_style_mod_template()
  %{<% for @icon in @icons -%>
pub mod <%= @icon %>;
<% end %>}
end



class LibRsFile
  include ERB::Util
  attr_accessor :template, :styles
  def initialize(template, styles)
    @template = template
    @styles = styles
  end

  def render()
    ERB.new(@template, nil, '-').result(binding)
  end
  def save(file)
    File.open(file, "w+") do |f|
      f.write(render)
    end
  end
end

class StyleModRsFile
  include ERB::Util
  attr_accessor :template, :icons
  def initialize(template, icons)
    @template = template
    @icons = icons
  end

  def render()
    ERB.new(@template, nil, '-').result(binding)
  end
  def save(file)
    File.open(file, "w+") do |f|
      f.write(render)
    end
  end
end

class IconRsFile
  include ERB::Util
  attr_accessor :template, :struct_name, :svg
  def initialize(struct_name, svg)
    @template = File.read("icon.erb")
    @struct_name = struct_name
    @svg = svg
  end

  def render()
    ERB.new(@template, nil, '-').result(binding)
  end
  def save(file)
    File.open(file, "w+") do |f|
      f.write(render)
    end
  end
end


a = Dir.glob("material-design-icons/src/**/24px.svg").inject({}) do |c,path|
  segments = Pathname(path).each_filename.to_a
  group = segments[-4]
  icon_name = "icon_#{segments[-3]}"
  struct_name = "icon_#{segments[-3].capitalize}".split('_').collect(&:capitalize).join
  style = segments[-2]

  mod_folder = "src/#{style}/#{group}/"
  mod_path = "#{mod_folder}/#{icon_name}.rs"

  FileUtils.mkdir_p(mod_folder)
  svg = File.read(path)
    .gsub(/<title>(.+)<\/title>/) {|title| "{ \"#{title}\" }"}
    .gsub(/xmlns:xlink="http:\/\/www\.w3\.org\/1999\/xlink"/, "")
    .gsub(/xlink:href="#SVGID_1_"/, "")

  IconRsFile.new(struct_name, svg).save(mod_path)

  c[style] ||= {}
  c[style][group] ||= []
  c[style][group] << icon_name
  c
end




styles = a.keys

puts a

LibRsFile.new(get_lib_template, styles).save("src/lib.rs")

styles.each do |style|
  StyleModRsFile.new(get_style_mod_template, a[style].keys).save("src/#{style}.rs")
  a[style].keys.each do |group|
    StyleModRsFile.new(get_style_mod_template, a[style][group]).save("src/#{style}/#{group}.rs")
  end
end
