# frozen_string_literal: true

require 'json'

RSpec.describe DiceNomShim do
  it "has a version number" do
    expect(DiceNomShim::VERSION).not_to be nil
  end

  it "does something useful" do
    hsh = JSON.parse(DiceNomShim.roll("2d6"))
    expect(hsh[0].dig("lhs", "total")).to be >= 2
  end
end
