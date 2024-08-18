# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List
from openapi_client.models.sapi_v1_margin_account_get200_response_user_assets_inner import SapiV1MarginAccountGet200ResponseUserAssetsInner
from openapi_client.models.sapi_v1_sub_account_margin_account_get200_response_margin_trade_coeff_vo import SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo
from typing import Optional, Set
from typing_extensions import Self

class SapiV1SubAccountMarginAccountGet200Response(BaseModel):
    """
    SapiV1SubAccountMarginAccountGet200Response
    """ # noqa: E501
    email: StrictStr
    margin_level: StrictStr = Field(alias="marginLevel")
    total_asset_of_btc: StrictStr = Field(alias="totalAssetOfBtc")
    total_liability_of_btc: StrictStr = Field(alias="totalLiabilityOfBtc")
    total_net_asset_of_btc: StrictStr = Field(alias="totalNetAssetOfBtc")
    margin_trade_coeff_vo: SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo = Field(alias="marginTradeCoeffVo")
    margin_user_asset_vo_list: List[SapiV1MarginAccountGet200ResponseUserAssetsInner] = Field(alias="marginUserAssetVoList")
    __properties: ClassVar[List[str]] = ["email", "marginLevel", "totalAssetOfBtc", "totalLiabilityOfBtc", "totalNetAssetOfBtc", "marginTradeCoeffVo", "marginUserAssetVoList"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SapiV1SubAccountMarginAccountGet200Response from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of margin_trade_coeff_vo
        if self.margin_trade_coeff_vo:
            _dict['marginTradeCoeffVo'] = self.margin_trade_coeff_vo.to_dict()
        # override the default output from pydantic by calling `to_dict()` of each item in margin_user_asset_vo_list (list)
        _items = []
        if self.margin_user_asset_vo_list:
            for _item in self.margin_user_asset_vo_list:
                if _item:
                    _items.append(_item.to_dict())
            _dict['marginUserAssetVoList'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1SubAccountMarginAccountGet200Response from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "email": obj.get("email"),
            "marginLevel": obj.get("marginLevel"),
            "totalAssetOfBtc": obj.get("totalAssetOfBtc"),
            "totalLiabilityOfBtc": obj.get("totalLiabilityOfBtc"),
            "totalNetAssetOfBtc": obj.get("totalNetAssetOfBtc"),
            "marginTradeCoeffVo": SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo.from_dict(obj["marginTradeCoeffVo"]) if obj.get("marginTradeCoeffVo") is not None else None,
            "marginUserAssetVoList": [SapiV1MarginAccountGet200ResponseUserAssetsInner.from_dict(_item) for _item in obj["marginUserAssetVoList"]] if obj.get("marginUserAssetVoList") is not None else None
        })
        return _obj


