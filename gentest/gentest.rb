#!/usr/bin/env ruby

require 'watir'
require 'fileutils'

caps = Selenium::WebDriver::Remote::Capabilities.chrome(
  "loggingPrefs"=>{
    "browser"=>"ALL",
    "performance"=>"ALL"
  }
)
browser = Watir::Browser.new(:chrome, :desired_capabilities => caps, :switches => ['--force-device-scale-factor=1', '--window-position=0,0'])

Dir.chdir(File.dirname($0))

Dir['../src/yoga/gentest/fixtures/*.html'].each do |file|
  fixture = File.read(file)
  name = File.basename(file, '.*')
  puts "Generate #{name}"

  ltr_fixture = fixture.gsub('start', 'left')
                       .gsub('end', 'right')
                       .gsub('flex-left', 'flex-start')
                       .gsub('flex-right', 'flex-end')

  rtl_fixture = fixture.gsub('start', 'right')
                       .gsub('end', 'left')
                       .gsub('flex-right', 'flex-start')
                       .gsub('flex-left', 'flex-end')

  template = File.open('test-template.html').read
  f = File.open('test.html', 'w')
  f.write sprintf(template, name, ltr_fixture, rtl_fixture, fixture)
  f.close
  FileUtils.copy('test.html', "#{name}.html") if $DEBUG

  browser.goto('file://' + Dir.pwd + '/test.html')
  logs = browser.driver.manage.logs.get(:browser)
  puts logs[1]
  
  f = File.open("../tests/#{name[2..-1].gsub!(/(.)([A-Z])/,'\1_\2').downcase}.rs", 'w')
  f.write eval(logs[0].message.sub(/^[^"]*/, '')).sub('YogaTest', name)
  f.close
end
File.delete('test.html')
browser.close
